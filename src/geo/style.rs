use std::collections::HashMap;
use osmpbf::TagIter;
use serde::{Deserialize, Serialize};
use skia_safe::RGB;
use crate::geo::map_generator::{Tag, WayData};
use crate::motion_graphics::elements::Element;
use crate::motion_graphics::elements::shape::Shape;

pub struct MapStyleSettings{
    pub way: HashMap<String,WayStyleSettings>,
    pub area: HashMap<String,AreaStyleSettings>,
    pub building: HashMap<String,AreaStyleSettings>,
}



pub struct WayStyleSettings{
    pub is_enabled: bool,
    pub(crate) width: f32,
    pub(crate) color: RGB,
    ///defines a scale when the value will be displayed when rendering the map
    pub(crate) render_threshold: Option<f32>,

}

pub struct AreaStyleSettings{
    pub is_enabled: bool,
    pub(crate) color: RGB,

}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Category{
    NotSpecified,
    Point,
    Path,
    Area,
    Building,
}

impl WayStyleSettings{
    pub fn new(width: f32, color: RGB, render_threshold: Option<f32>) -> WayStyleSettings{
        WayStyleSettings{
            is_enabled: true,
            width,
            color,
            render_threshold,
        }
    }
}

impl AreaStyleSettings{
    pub fn new(color: RGB) -> AreaStyleSettings{
        AreaStyleSettings{
            is_enabled: true,
            color
        }
    }
}

impl Default for MapStyleSettings{
    fn default()->Self{
        let mut way = HashMap::new();

        let road_color = RGB{ r: 180, g: 180, b: 180};

        way.insert(String::from("motorway"),
                   WayStyleSettings::new(3f32,road_color,Some(6f32)));

        way.insert(String::from("trunk"),
                   WayStyleSettings::new(3f32,road_color,Some(6f32)));

        way.insert(String::from("primary"),
                   WayStyleSettings::new(3f32,road_color,Some(6f32)));

        way.insert(String::from("secondary"),
                   WayStyleSettings::new(2f32,road_color,Some(6f32)));

        way.insert(String::from("tertiary"),
                   WayStyleSettings::new(2f32,road_color,Some(7f32)));

        way.insert(String::from("unclassified"),
                   WayStyleSettings::new(1f32,road_color,Some(7f32)));

        way.insert(String::from("residential"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));


        way.insert(String::from("motorway_link"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("trunk_link"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("primary_link"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("secondary_link"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("tertiary_link"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));


        way.insert(String::from("living_street"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("service"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("pedestrian"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("track"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));





        let mut area_settings = HashMap::new();

        //oklch(0.5297 0.0851 202.43)
        area_settings.insert(String::from("water"),
                                 AreaStyleSettings::new(RGB{r: 22 , g: 122, b: 129}));

        //oklch(0.5297 0.0851 138.34)
        area_settings.insert(String::from("forest"),
                             AreaStyleSettings::new(RGB{r: 82 , g: 119, b: 70}));

        //oklch(0.6602 0.0851 138.34)
        area_settings.insert(String::from("grassland"),
                             AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        //oklch(0.5297 0.0851 138.34)
        area_settings.insert(String::from("farmland"),
                             AreaStyleSettings::new(RGB{r: 136 , g: 155, b: 96}));

        //oklch(0.6602 0.0686 105.22)
        area_settings.insert(String::from("residential"),
                             AreaStyleSettings::new(RGB{r: 152 , g: 149, b: 99}));

        //oklch(0.6602 0.0686 105.22)
        area_settings.insert(String::from("construction"),
                             AreaStyleSettings::new(RGB{r: 95 , g: 95, b: 95}));

        //oklch(0.6602 0.0686 31.51)
        area_settings.insert(String::from("commercial"),
                             AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));

        //oklch(0.6602 0.0686 31.51)
        area_settings.insert(String::from("industrial"),
                             AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));

        //oklch(0.6602 0.0686 31.51)
        area_settings.insert(String::from("retail"),
                             AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));


        let mut building = HashMap::new();

        building.insert(String::from("yes"),
                             AreaStyleSettings::new(RGB{r:100, g:100, b:100}));

        MapStyleSettings{
            way,
            area: area_settings,
            building,
        }
    }
}

impl MapStyleSettings{
    pub fn filter_by_tag(&self, key: &str) -> bool{
        match key {
            "highway" => true,
            "natural" => true,
            "landuse" => true,
            "building" => true,
            _ => false
        }
    }

    pub fn filter_by_value(&self, value: &str) -> bool{
        if self.way.contains_key(&value.to_string()) {
            self.way[value].is_enabled
        } else if self.area.contains_key(&value.to_string()) {
            self.area[value].is_enabled
        } else if self.building.contains_key(&value.to_string()) {
            self.building[value].is_enabled
        } else {
            false
        }
    }

    pub fn map_tag_to_category(&self, tag: &str, content: &str) -> Option<Tag>{
        let mut _category: Category = Category::NotSpecified;
        let mut _type = String::new();
        match tag{
            "highway" => { _category = Category::Path; }
            "natural" => { _category = Category::Area; }
            "landuse" => { _category = Category::Area; }
            "building" => { _category = Category::Building; }
            _ => { return None;  }
        }

        Some(Tag::new(_category, content.to_string()))
    }

    pub fn map_way_data_to_shape(&self, way_data: &WayData) -> Shape{
        todo!()
    }
}

pub trait Enable {
    fn is_enabled(&self) -> bool;
}

impl Enable for WayStyleSettings{
    fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}
impl Enable for AreaStyleSettings{
    fn is_enabled(&self) -> bool{
        self.is_enabled
    }
}

