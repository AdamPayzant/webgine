use bytemuck;
use glyphon;
use std::sync::Arc;
use wgpu::{self, util::DeviceExt};
use winit::{event::WindowEvent, window::Window};

mod vertex;
use vertex::Vertex;

fn create_rect_vertices(pos: [f32; 2], size: [f32; 2], color: [f32; 4]) -> [Vertex; 6] {
    let [x, y] = pos;
    let [w, h] = size;

    [
        Vertex {
            position: [x, y],
            color,
        },
        Vertex {
            position: [x, y + h],
            color,
        },
        Vertex {
            position: [x + w, y],
            color,
        },
        Vertex {
            position: [x + w, y],
            color,
        },
        Vertex {
            position: [x, y + h],
            color,
        },
        Vertex {
            position: [x + w, y + h],
            color,
        },
    ]
}

#[derive(Clone, Debug)]
pub enum GFXRenderCommand {
    Rect {
        position: [f32; 2],
        size: [f32; 2],
        color: [f32; 4],
    },
    Text {
        position: [f32; 2],
        content: String,
        color: [f32; 4],
    },
}

pub struct GFXState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,

    render_pipeline: wgpu::RenderPipeline,
    render_commands: Vec<GFXRenderCommand>,

    frame_view: wgpu::TextureView,

    vertex_buffer: wgpu::Buffer,

    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer,
}

impl<'a> GFXState<'a> {
    pub async fn new(window: Arc<Window>) -> GFXState<'a> {
        let size = window.inner_size();

        // This is to get around RADV drivers having comformanceVersion  0.0.0.0
        let mut instance_flags = wgpu::InstanceFlags::default();
        instance_flags.set(
            wgpu::InstanceFlags::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER,
            true,
        );

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: instance_flags,
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_formats = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_formats,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/shader.wgsl")).into(),
            ),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let mut render_commands = Vec::new();

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: std::mem::size_of::<Vertex>() as u64 * 6,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // TODO: Switch this to bgr
        let swapchain_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let mut font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(&device);
        let viewport = glyphon::Viewport::new(&device, &cache);
        let mut atlas = glyphon::TextAtlas::new(&device, &queue, &cache, swapchain_format);
        let text_renderer = glyphon::TextRenderer::new(
            &mut atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );
        let mut text_buffer =
            glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(30.0, 42.0));
        text_buffer.set_size(
            &mut font_system,
            Some((config.width as f64 * window.scale_factor()) as f32),
            Some((config.height as f64 * window.scale_factor()) as f32),
        );

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d::default(),
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB, so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("diffuse_texture"),
            // This is the same as with the SurfaceConfig. It
            // specifies what texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case) is
            // always supported. Note that using a different
            // texture format is not supported on the WebGL2
            // backend.
            view_formats: &[],
        });
        let frame_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

        render_commands.push(GFXRenderCommand::Rect {
            position: [0.5, 0.2],
            size: [0.4, 0.4],
            color: [0.0, 0.0, 0.0, 0.0],
        });
        render_commands.push(GFXRenderCommand::Text {
            position: [0.2, 0.2],
            content: "Hello World".to_owned(),
            color: [0.0, 0.0, 0.0, 0.0],
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            render_commands,

            vertex_buffer,

            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,

            frame_view,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        // todo!()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("UI Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view, // <- use actual surface view
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        for cmd in self.render_commands.clone() {
            match cmd {
                GFXRenderCommand::Rect {
                    position,
                    size,
                    color,
                } => {
                    let vertices = create_rect_vertices(position, size, color);
                    self.queue.write_buffer(
                        &mut self.vertex_buffer,
                        0,
                        bytemuck::cast_slice(&vertices),
                    );

                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    render_pass.draw(0..6, 0..1);
                }
                GFXRenderCommand::Text {
                    position,
                    content,
                    color,
                } => {
                    self.text_buffer.set_text(
                        &mut self.font_system,
                        &content,
                        glyphon::Attrs::new().family(glyphon::Family::SansSerif),
                        glyphon::Shaping::Advanced,
                    );

                    match self.text_renderer.prepare(
                        &self.device,
                        &self.queue,
                        &mut self.font_system,
                        &mut self.atlas,
                        &self.viewport,
                        [glyphon::TextArea {
                            buffer: &self.text_buffer,
                            left: position[0],
                            top: position[1],
                            scale: 32.0,
                            bounds: glyphon::TextBounds {
                                left: 0,
                                top: 0,
                                right: self.window.inner_size().width as i32,
                                bottom: self.window.inner_size().height as i32,
                            },
                            default_color: glyphon::Color::rgba(
                                color[1] as u8,
                                color[2] as u8,
                                color[3] as u8,
                                color[0] as u8,
                            ),
                            custom_glyphs: &[],
                        }],
                        &mut self.swash_cache,
                    ) {
                        Ok(_) => {}
                        Err(_) => return Err(wgpu::SurfaceError::Other),
                    };

                    match self
                        .text_renderer
                        .render(&self.atlas, &self.viewport, &mut render_pass)
                    {
                        Ok(_) => {}
                        Err(_) => return Err(wgpu::SurfaceError::Other),
                    };
                }
            }
        }

        drop(render_pass);
        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}
