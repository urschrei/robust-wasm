use robust::{incircle, insphere, orient2d, orient3d, Coord, Coord3D};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// WASM version of [orient2d]
pub fn worient2d(pax: f64, pay: f64, pbx: f64, pby: f64, pcx: f64, pcy: f64) -> f64 {
    orient2d(
        Coord { x: pax, y: pay },
        Coord { x: pbx, y: pby },
        Coord { x: pcx, y: pcy },
    )
}

#[wasm_bindgen]
/// WASM version of [orient3d]
pub fn worient3d(
    pax: f64,
    pay: f64,
    paz: f64,
    pbx: f64,
    pby: f64,
    pbz: f64,
    pcx: f64,
    pcy: f64,
    pcz: f64,
    pdx: f64,
    pdy: f64,
    pdz: f64,
) -> f64 {
    orient3d(
        Coord3D {
            x: pax,
            y: pay,
            z: paz,
        },
        Coord3D {
            x: pbx,
            y: pby,
            z: pbz,
        },
        Coord3D {
            x: pcx,
            y: pcy,
            z: pcz,
        },
        Coord3D {
            x: pdx,
            y: pdy,
            z: pdz,
        },
    )
}

#[wasm_bindgen]
/// WASM version of [incircle]
pub fn wincircle(
    pax: f64,
    pay: f64,
    pbx: f64,
    pby: f64,
    pcx: f64,
    pcy: f64,
    pdx: f64,
    pdy: f64,
) -> f64 {
    incircle(
        Coord { x: pax, y: pay },
        Coord { x: pbx, y: pby },
        Coord { x: pcx, y: pcy },
        Coord { x: pdx, y: pdy },
    )
}

#[wasm_bindgen]
/// WASM version of [insphere]
pub fn winsphere(
    pax: f64,
    pay: f64,
    paz: f64,
    pbx: f64,
    pby: f64,
    pbz: f64,
    pcx: f64,
    pcy: f64,
    pcz: f64,
    pdx: f64,
    pdy: f64,
    pdz: f64,
    pex: f64,
    pey: f64,
    pez: f64,
) -> f64 {
    insphere(
        Coord3D {
            x: pax,
            y: pay,
            z: paz,
        },
        Coord3D {
            x: pbx,
            y: pby,
            z: pbz,
        },
        Coord3D {
            x: pcx,
            y: pcy,
            z: pcz,
        },
        Coord3D {
            x: pdx,
            y: pdy,
            z: pdz,
        },
        Coord3D {
            x: pex,
            y: pey,
            z: pez,
        },
    )
}
