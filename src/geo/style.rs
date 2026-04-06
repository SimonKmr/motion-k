use crate::geo::map_generator::Tag;
use crate::motion_graphics::attributes::attribute::Attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::line::Line;
use crate::motion_graphics::elements::shape::Shape;
use serde::{Deserialize, Serialize};
use skia_safe::RGB as SkiaRGB;
use std::collections::HashMap;
use vector2d::Vector2D;
use crate::motion_graphics::elements::Element;

pub trait Style {
    fn element(
        &self,
        position: Box<dyn Attribute<Vector2D<f32>> + 'static>,
        points: Vec<Box<dyn Attribute<Vector2D<f32>> + 'static>> ) -> Box<dyn Element> ;

    fn render_threshold(&self) -> Option<f32>;
}

#[derive(Serialize, Deserialize)]
pub struct MapStyleSettings{
    pub way: HashMap<String,WayStyleSettings>,
    pub area: HashMap<String,AreaStyleSettings>,
    pub building: HashMap<String,AreaStyleSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct WayStyleSettings{
    pub is_enabled: bool,
    pub(crate) width: f32,
    pub(crate) color: RGB,
    ///defines a scale when the value will be displayed when rendering the map
    pub(crate) render_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize, Copy, Clone,PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize)]
pub struct AreaStyleSettings{
    pub is_enabled: bool,
    pub(crate) color: RGB,
    pub(crate) render_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize, Copy, Clone,PartialEq)]
pub enum Category{
    NotSpecified,
    Point,
    Path,
    Water,
    Area,
    Building,
}

impl RGB {
    pub fn into_skia_rgb(self) -> SkiaRGB {
        SkiaRGB {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
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

impl Style for WayStyleSettings{
    fn element(
        &self, position: Box<dyn Attribute<Vector2D<f32>>>,
        points: Vec<Box<dyn Attribute<Vector2D<f32>> + 'static>> ) -> Box<dyn Element> {
        Box::new(Line {
            position_offset: position,
            start: 0f32.into_bsa(),
            end: 1f32.into_bsa(),
            width: self.width.into_bsa(),
            color: self.color.into_skia_rgb().into_bsa(),
            stroke_caps: skia_safe::paint::Cap::Round,
            is_antialias: true,
            points,
        })
    }

    fn render_threshold(&self) -> Option<f32> {
        self.render_threshold.clone()
    }
}

impl Style for AreaStyleSettings{
    fn element(&self, position: Box<dyn Attribute<Vector2D<f32>> + 'static>, points: Vec<Box<dyn Attribute<Vector2D<f32>> + 'static>>) -> Box<dyn Element> {
        Box::new(Shape {
            position_offset: position,
            color: self.color.into_skia_rgb().into_bsa(),
            is_antialias: true,
            points,
        })
    }

    fn render_threshold(&self) -> Option<f32> {
        self.render_threshold.clone()
    }
}

impl AreaStyleSettings{
    pub fn new(color: RGB) -> AreaStyleSettings{
        AreaStyleSettings{
            is_enabled: true,
            color,
            render_threshold: None,
        }
    }

    /// new with threshold
    pub fn new_wt(color: RGB, threshold: f32) -> AreaStyleSettings{
        AreaStyleSettings{
            is_enabled: true,
            color,
            render_threshold: Some(threshold),
        }
    }

    pub fn element(
        &self,
        position: Box<dyn Attribute<Vector2D<f32>> + 'static>,
        points: Vec<Box<dyn Attribute<Vector2D<f32>> + 'static>> ) -> Shape{
        Shape {
            position_offset: position,
            color: self.color.into_skia_rgb().into_bsa(),
            is_antialias: true,
            points,
        }
    }
}

impl Default for MapStyleSettings{
    fn default()->Self{
        let mut way = HashMap::new();

        let road_color = RGB{ r: 180, g: 180, b: 180};

        way.insert(String::from("motorway"),
                   WayStyleSettings::new(6f32,road_color,Some(6f32)));

        way.insert(String::from("trunk"),
                   WayStyleSettings::new(5f32,road_color,Some(6f32)));

        way.insert(String::from("primary"),
                   WayStyleSettings::new(5f32,road_color,Some(6f32)));

        way.insert(String::from("secondary"),
                   WayStyleSettings::new(4f32,road_color,Some(6f32)));

        way.insert(String::from("tertiary"),
                   WayStyleSettings::new(3f32,road_color,Some(7f32)));

        way.insert(String::from("unclassified"),
                   WayStyleSettings::new(2f32,road_color,Some(7f32)));

        way.insert(String::from("residential"),
                   WayStyleSettings::new(2f32,road_color,Some(8f32)));


        way.insert(String::from("motorway_link"),
                   WayStyleSettings::new(2f32,road_color,Some(8f32)));

        way.insert(String::from("trunk_link"),
                   WayStyleSettings::new(2f32,road_color,Some(8f32)));

        way.insert(String::from("primary_link"),
                   WayStyleSettings::new(2f32,road_color,Some(8f32)));

        way.insert(String::from("secondary_link"),
                   WayStyleSettings::new(2f32,road_color,Some(8f32)));

        way.insert(String::from("tertiary_link"),
                   WayStyleSettings::new(2f32,road_color,Some(8f32)));


        way.insert(String::from("living_street"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("service"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("pedestrian"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("track"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        way.insert(String::from("man_made"),
                   WayStyleSettings::new(1f32,road_color,Some(8f32)));

        //Water
        //oklch(0.5297 0.0851 202.43)
        way.insert(String::from("stream"),
                   WayStyleSettings::new(1f32,RGB{r: 22 , g: 122, b: 129},Some(7f32)));


        let mut area = HashMap::new();

        //oklch(0.5297 0.0851 202.43)
        area.insert(String::from("water"),
                    AreaStyleSettings::new(RGB{r: 22 , g: 122, b: 129}));

        //oklch(0.5297 0.0851 138.34)
        area.insert(String::from("forest"),
                    AreaStyleSettings::new(RGB{r: 82 , g: 119, b: 70}));

        //oklch(0.6602 0.0851 138.34)
        area.insert(String::from("grassland"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        //oklch(0.5297 0.0851 138.34)
        area.insert(String::from("farmland"),
                    AreaStyleSettings::new(RGB{r: 136 , g: 155, b: 96}));

        //oklch(0.6602 0.0244 44.33)
        area.insert(String::from("residential"),
                    AreaStyleSettings::new(RGB{r: 160 , g: 142, b: 134}));

        //oklch(0.6602 0.0686 105.22)
        area.insert(String::from("construction"),
                    AreaStyleSettings::new(RGB{r: 95 , g: 95, b: 95}));

        //oklch(0.6602 0.0686 31.51)
        area.insert(String::from("commercial"),
                    AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));

        //oklch(0.6602 0.0686 31.51)
        area.insert(String::from("industrial"),
                    AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));

        //oklch(0.6602 0.0686 31.51)
        area.insert(String::from("retail"),
                    AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));



        area.insert(String::from("wood"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        area.insert(String::from("railway"),
                    AreaStyleSettings::new(RGB{r: 146 , g: 146, b: 146}));

        area.insert(String::from("parking"),
                    AreaStyleSettings::new(RGB{r: 146 , g: 146, b: 146}));

        area.insert(String::from("quarry"),
                    AreaStyleSettings::new(RGB{r: 146 , g: 146, b: 146}));

        area.insert(String::from("cemetery"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        //leisure
        area.insert(String::from("park"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        area.insert(String::from("dog_park"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        area.insert(String::from("garden"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        area.insert(String::from("pitch"),
                    AreaStyleSettings::new(RGB{r: 89 , g: 163, b: 137}));

        area.insert(String::from("sports_center"),
                    AreaStyleSettings::new(RGB{r: 89 , g: 163, b: 137}));

        area.insert(String::from("stadium"),
                    AreaStyleSettings::new(RGB{r: 121 , g: 155, b: 141}));

        area.insert(String::from("swimming_pool"),
                    AreaStyleSettings::new(RGB{r: 22 , g: 122, b: 129}));

        area.insert(String::from("track"),
                    AreaStyleSettings::new(RGB{r: 89 , g: 163, b: 137}));

        //amentity
        area.insert(String::from("grave_yard"),
                    AreaStyleSettings::new(RGB{r: 92 , g: 119, b: 66}));

        //landuse
        area.insert(String::from("plant_nursery"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        area.insert(String::from("brownfield"),
                    AreaStyleSettings::new(RGB{r: 127 , g: 104, b: 92}));

        area.insert(String::from("allotments"),
                    AreaStyleSettings::new(RGB{r: 120 , g: 159, b: 108}));

        area.insert(String::from("basin"),
                   AreaStyleSettings::new(RGB{r: 22 , g: 122, b: 129}));


        //oklch(0.6602 0.0244 44.33)
        area.insert(String::from("school"),
                    AreaStyleSettings::new(RGB{r: 160 , g: 142, b: 134}));

        area.insert(String::from("grass"),
                    AreaStyleSettings::new(RGB{r: 151 , g: 191, b: 138}));

        area.insert(String::from("meadow"),
                    AreaStyleSettings::new(RGB{r: 104 , g: 143, b: 92}));

        area.insert(String::from("scrub"),
                    AreaStyleSettings::new(RGB{r: 90 , g: 127, b: 78}));

        //oklch(0.6602 0.0686 31.51)
        area.insert(String::from("farmyard"),
                    AreaStyleSettings::new(RGB{r: 184 , g: 131, b: 120}));



        let mut building = HashMap::new();

        building.insert(String::from("yes"),
                        AreaStyleSettings::new_wt(RGB{r:100, g:100, b:100}, 7f32));

        building.insert(String::from("house"),
                        AreaStyleSettings::new_wt(RGB{r:100, g:100, b:100}, 7f32));

        building.insert(String::from("garage"),
                        AreaStyleSettings::new_wt(RGB{r:100, g:100, b:100},7f32));

        MapStyleSettings{
            way,
            area,
            building,
        }
    }
}

impl MapStyleSettings{
    pub fn filter_by_tag(&self, key: &str) -> bool{
        match self.map_tag_to_category(key,""){
            None => { false }
            Some(_) => { true }
        }
    }

    pub fn filter_by_area_tag(&self, key: &str) -> bool{
        match self.map_tag_to_category(key,""){
            None => { false}
            Some(category) => { category.category == Category::Area }
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
        match tag{
            "highway" => { _category = Category::Path; }
            "man_made" => { _category = Category::Path; }
            "natural" => { _category = Category::Area; }
            "landuse" => { _category = Category::Area; }
            "amenity" => { _category = Category::Area; }
            "leisure" => { _category = Category::Area; }
            "building" => { _category = Category::Building; }
            "waterway" => { _category = Category::Water; }

            _ => { return None }
        }
        Some(Tag::new(_category, content.to_string()))
    }
}