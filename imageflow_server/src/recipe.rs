use std::fs::File;
use std::io::{Read, ErrorKind};
use iron::request::Request;
use super::ServerError;

pub fn get_framewise(req: &mut Request)
    -> Box<dyn Fn(s::ImageInfo) -> std::result::Result<s::Framewise, ServerError>> {

    let url: url::Url = req.url.clone().into();
    match parse_recipe(&url) {
        Some(recipe) => {
            Box::new(move |info: s::ImageInfo| {
                recipe_framewise(&info, &recipe)
            })
        }
        None => {
            Box::new(
                move |info: s::ImageInfo| {
                    ir4_framewise(&info, &url)
                }
            )
        }
    }
}

fn parse_recipe(url: &url::Url) -> Option<String> {
    for (key, value) in url.query_pairs() {
        let k = key.to_lowercase(); //Trim whitespace?
        let v = value.into_owned();

        if k == "recipe" {
            return Option::Some(v)
        }
    }
    Option::None
}


fn recipe_framewise(_info: &s::ImageInfo, recipe: &str) -> std::result::Result<s::Framewise, ServerError> {
    let mut data = Vec::new();
    let mut f = File::open(recipe)?;
    f.read_to_end(&mut data)?;
    let parsed: s::Build001 = serde_json::from_slice(&data).map_err(|e |{
        std::io::Error::new(ErrorKind::InvalidData, e)
    })?;
    Result::Ok(parsed.framewise)
}

fn ir4_framewise(_info: &s::ImageInfo, url: &url::Url) -> std::result::Result<s::Framewise, ServerError> {
    let t = ::imageflow_riapi::ir4::Ir4Translate{
        i: ::imageflow_riapi::ir4::Ir4Command::Url(url.as_str().to_owned()),
        decode_id: Some(0),
        encode_id: Some(1),
        watermarks: None
    };
    t.translate().map_err( ServerError::LayoutSizingError).and_then(|r: ::imageflow_riapi::ir4::Ir4Result| Ok(s::Framewise::Steps(r.steps.unwrap())))
}

// fn predef_json_setup(mount: &MountLocation) -> Result<(PathBuf, EngineHandler<PathBuf>), String> {
//     if mount.engine_args.len() < 1 {
//         Err("predef_json requires at least one argument - the path to the physical folder of json scripts".to_owned())
//     } else {
//         //TODO: validate path
//         let local_dir = Path::new(&mount.engine_args[0]).canonicalize().map_err(|e| format!("{:?} for {:?}", e, &mount.engine_args[0]))?;
//         Ok((local_dir, predef_json_handler))
//     }
// }
