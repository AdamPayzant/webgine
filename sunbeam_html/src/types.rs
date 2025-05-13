#[derive(Debug)]
pub enum UnitType {
    // Abolute lengths
    Cm,
    Mm,
    In,
    Px,
    Pt,
    Pc,
    // Relative Lengths
    Em,
    Ex,
    Ch,
    Rem,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Percent,
}

#[derive(Debug)]
pub struct Unit {
    utype: UnitType,
    value: i64,
}

#[derive(Debug)]
pub struct PositioningParamters {
    top: Unit,
    bottom: Unit,
    Left: Unit,
    Right: Unit,
}

#[derive(Default, Debug)]
pub enum Positioning {
    #[default]
    Static,
    Relative(PositioningParamters),
    Fixed(PositioningParamters),
    Absolute(PositioningParamters),
    Sticky(PositioningParamters),
}
