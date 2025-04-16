use bytemuck;
use glyphon;
use std::sync::Arc;
use wgpu;
use winit::{event::WindowEvent, window::Window};

mod utils;
mod vertex;
use vertex::Vertex;

pub mod render_cmds;
use render_cmds::GFXRenderCommand;

pub struct GFXState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,

    render_pipeline: wgpu::RenderPipeline,
    render_commands_outer: Vec<GFXRenderCommand>, // For the outer UI of the browser
    render_commands_inner: Vec<GFXRenderCommand>, // For the contents of the pane

    vertex_buffers: Vec<wgpu::Buffer>,

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

        let mut render_commands_outer = Vec::new();
        let mut render_commands_inner = Vec::new();

        let vertex_buffer = utils::new_vertex_buffer(&device);

        let swapchain_format = wgpu::TextureFormat::Bgra8UnormSrgb;

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

        render_commands_inner.push(GFXRenderCommand::Rect {
            position: [0.5, 0.2],
            size: [0.4, 0.4],
            color: [1.0, 0.0, 0.0, 0.0],
        });
        render_commands_inner.push(GFXRenderCommand::Rect {
            position: [-0.5, -0.2],
            size: [0.4, 0.4],
            color: [1.0, 0.0, 1.0, 0.0],
        });
        render_commands_inner.push(GFXRenderCommand::Outline {
            position: [0.0, 0.0],
            size: [0.4, 0.4],
            thickness: 0.2,
            color: [1.0, 0.0, 0.0, 0.0],
        });
        render_commands_inner.push(GFXRenderCommand::Text {
            position: [0.1, 0.1],
            content: "Hello World".to_owned(),
            color: [0.0, 0.0, 0.0, 1.0],
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            render_commands_outer,
            render_commands_inner,

            vertex_buffers: vec![vertex_buffer],

            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
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
                view: &view,
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

        self.viewport.update(
            &self.queue,
            glyphon::Resolution {
                width: self.window.inner_size().width,
                height: self.window.inner_size().height,
            },
        );

        render_pass.set_pipeline(&self.render_pipeline);

        let render_cmds = [
            self.render_commands_outer.clone(),
            self.render_commands_inner.clone(),
        ]
        .concat();

        let mut cur_vert_buffer: usize = 0;
        let mut offset = 0;

        for cmd in render_cmds {
            match cmd {
                GFXRenderCommand::Rect {
                    position,
                    size,
                    color,
                } => {
                    if !(offset < utils::VERTEX_BUFFER_MAX_RECT) {
                        cur_vert_buffer += 1;
                        offset = 0;
                    }

                    let mut buffer = match self.vertex_buffers.get_mut(cur_vert_buffer) {
                        Some(v) => v,
                        None => {
                            self.vertex_buffers
                                .push(utils::new_vertex_buffer(&self.device));
                            match self.vertex_buffers.last_mut() {
                                Some(v) => v,
                                None => {
                                    // ERROR
                                    return Err(wgpu::SurfaceError::Other);
                                }
                            }
                        }
                    };

                    let vertices = utils::create_rect_vertices(position, size, color);
                    let byte_offset = utils::compute_vertex_buffer_offset(offset);
                    let vertex_data_size = std::mem::size_of::<Vertex>() * 6;
                    self.queue.write_buffer(
                        &mut buffer,
                        byte_offset,
                        bytemuck::cast_slice(&vertices),
                    );

                    render_pass.set_vertex_buffer(
                        0,
                        buffer.slice(byte_offset..byte_offset + vertex_data_size as u64),
                    );
                    render_pass.draw(0..6, 0..1);
                    offset += 1;
                }
                GFXRenderCommand::Outline {
                    position,
                    size,
                    thickness,
                    color,
                } => {
                    let vertices = utils::create_outline(position, size, thickness, color);
                    for vert in vertices {
                        if !(offset < utils::VERTEX_BUFFER_MAX_RECT) {
                            cur_vert_buffer += 1;
                            offset = 0;
                        }

                        let mut buffer = match self.vertex_buffers.get_mut(cur_vert_buffer) {
                            Some(v) => v,
                            None => {
                                self.vertex_buffers
                                    .push(utils::new_vertex_buffer(&self.device));
                                match self.vertex_buffers.last_mut() {
                                    Some(v) => v,
                                    None => {
                                        // ERROR
                                        return Err(wgpu::SurfaceError::Other);
                                    }
                                }
                            }
                        };

                        let byte_offset = utils::compute_vertex_buffer_offset(offset);
                        let vertex_data_size = std::mem::size_of::<Vertex>() * 6;

                        self.queue.write_buffer(
                            &mut buffer,
                            utils::compute_vertex_buffer_offset(offset),
                            bytemuck::cast_slice(&vert),
                        );

                        render_pass.set_vertex_buffer(
                            0,
                            buffer.slice(byte_offset..byte_offset + vertex_data_size as u64),
                        );
                        render_pass.draw(0..6, 0..1);
                        offset += 1;
                    }
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
                            scale: 1.0,
                            bounds: glyphon::TextBounds {
                                left: 0,
                                top: 0,
                                right: self.window.inner_size().width as i32,
                                bottom: self.window.inner_size().height as i32,
                            },
                            default_color: utils::float_colors_to_glyphon_rgba(color),
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
        self.atlas.trim();

        Ok(())
    }

    pub fn set_outer_render_cmds(&mut self, cmds: Vec<GFXRenderCommand>) {
        self.render_commands_outer = cmds;
    }

    pub fn set_inner_render_cmds(&mut self, cmds: Vec<GFXRenderCommand>) {
        self.render_commands_inner = cmds;
    }
}
