
mod img;
mod item;

use common::obj::*;
use tomlinput::TomlInput;
use error::*;
use self::img::*;
use self::item::build_item_object;

pub fn build_object(tomlinput: TomlInput) -> Result<Object> {
    let object_type = tomlinput.object_type.clone();
    match object_type.as_ref() {
        "anim_img" => {
            return build_anim_img_object(tomlinput).map(|o| Object::AnimImg(o));
        }
        "chara_template" => {
            return build_chara_template_object(tomlinput).map(|o| Object::CharaTemplate(o));
        }
        "deco" => {
            return build_deco_object(tomlinput).map(|o| Object::Deco(o));
        }
        "item" => {
            return build_item_object(tomlinput).map(|o| Object::Item(o));
        }
        "special_tile" => {
            return build_special_tile_object(tomlinput).map(|o| Object::SpecialTile(o));
        }
        "tile" => {
            return build_tile_object(tomlinput).map(|o| Object::Tile(o));
        }
        "ui_img" => {
            return build_ui_img_object(tomlinput).map(|o| Object::UIImg(o));
        }
        "wall" => {
            return build_wall_object(tomlinput).map(|o| Object::Wall(o));
        }
        "talk_script" => {
            return build_talk_script_object(tomlinput).map(|o| Object::TalkScript(o));
        }
        _ => {
            bail!("Unknown object_type");
        }
    }
}

fn build_deco_object(tomlinput: TomlInput) -> Result<DecoObject> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(DecoObject {
        id: tomlinput.id,
        img: build_img(img)?,
    })
}

fn build_special_tile_object(tomlinput: TomlInput) -> Result<SpecialTileObject> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(SpecialTileObject {
        id: tomlinput.id,
        img: build_img(img)?,
    })
}

fn build_tile_object(tomlinput: TomlInput) -> Result<TileObject> {
    let tile_dep_input = get_optional_field!(tomlinput, tile);
    let img = get_optional_field!(tomlinput, image);
    
    Ok(TileObject {
        id: tomlinput.id,
        img: build_img(img)?,
        kind: tile_dep_input.kind,
    })
}

fn build_ui_img_object(tomlinput: TomlInput) -> Result<UIImgObject> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(UIImgObject {
        id: tomlinput.id,
        img: build_img(img)?,
    })
}

fn build_wall_object(tomlinput: TomlInput) -> Result<WallObject> {
    let img = get_optional_field!(tomlinput, image);
    
    Ok(WallObject {
        id: tomlinput.id,
        img: build_img(img)?,
    })
}

fn build_chara_template_object(tomlinput: TomlInput) -> Result<CharaTemplateObject> {    
    let chara_dep_input = get_optional_field!(tomlinput, chara_template);
    let img = get_optional_field!(tomlinput, image);
    
    Ok(CharaTemplateObject {
        id: tomlinput.id,
        img: build_img(img)?,
        race: chara_dep_input.race,
        gen_weight: chara_dep_input.gen_weight,
        gen_level: chara_dep_input.gen_level,
        max_hp: chara_dep_input.max_hp,
        str: chara_dep_input.str,
        vit: chara_dep_input.vit,
        dex: chara_dep_input.dex,
        int: chara_dep_input.int,
        wil: chara_dep_input.wil,
        cha: chara_dep_input.cha,
        spd: chara_dep_input.spd,
    })
}

fn build_anim_img_object(tomlinput: TomlInput) -> Result<AnimImgObject> {
    let anim_img_dep = get_optional_field!(tomlinput, anim_img);
    let img = get_optional_field!(tomlinput, image);

    Ok(AnimImgObject {
        id: tomlinput.id,
        img: build_img(img)?,
        duration: anim_img_dep.duration,
        n_frame: anim_img_dep.n_frame,
    })
}

fn build_talk_script_object(tomlinput: TomlInput) -> Result<TalkScriptObject> {
    let talk_script_dep = get_optional_field!(tomlinput, talk_script);
    use std::collections::HashMap;
    use common::talkscript::TalkSection;
    let mut sections: HashMap<String, TalkSection> = HashMap::new();
    for (k, v) in talk_script_dep.sections {
        let text = if v.text.is_none() && (v.is_empty.is_none() || v.is_empty.unwrap()) {
            // Setting default text id
            Some(format!("{}.{}", &tomlinput.id, k))
        } else {
            v.text
        };
        let sub_reaction = if let Some(sub_reaction) = v.sub_reaction {
            sub_reaction
        } else {
            Vec::new()
        };
        sections.insert(
            k,
            TalkSection {
                text: text,
                reaction: v.reaction,
                sub_reaction: sub_reaction,
            }
        );
    }

    Ok(TalkScriptObject {
        id: tomlinput.id,
        sections: sections,
    })
}

