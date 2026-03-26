use std::collections::HashMap;
use skia_safe::RGB;

pub struct MapStyleSettings{
    pub way_settings: HashMap<String,WayStyleSettings>
}

pub struct WayStyleSettings{
    pub(crate) width: f32,
    pub(crate) color: RGB,

}

impl WayStyleSettings{
    pub fn new(width: f32, color: RGB) -> WayStyleSettings{
        WayStyleSettings{
            width,
            color,
        }
    }
}

impl Default for MapStyleSettings{
    fn default()->Self{
        let mut way_settings_map = HashMap::new();

        way_settings_map.insert(String::from("motorway"),
                                WayStyleSettings::new(5f32,RGB{ r: 45, g: 120, b: 196}));

        way_settings_map.insert(String::from("trunk"),
                                WayStyleSettings::new(5f32,RGB{ r: 215, g: 181, b: 63}));

        way_settings_map.insert(String::from("primary"),
                                WayStyleSettings::new(5f32,RGB{ r: 215, g: 181, b: 63}));

        MapStyleSettings{
            way_settings: way_settings_map
        }
    }
}

