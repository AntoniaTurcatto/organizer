pub mod filter{
    pub const FILTER_EXT: &str = "ext";
    pub const FILTER_DATE: &str = "date";
    pub enum Filter{
        Ext,
        Date(DateFilter),
    }

    pub enum DateFilter{
        Day,
        Month,
        Year,
    }

    impl Filter{
        pub fn build<'a>(filter: &str) -> Self{
            if filter != FILTER_EXT && filter != FILTER_DATE{
                panic!("Invalid filter value");
            }

            if filter == FILTER_EXT {
                Filter::Ext
            } else{
                Filter::Date(DateFilter::Day)
            }
        }
    }

    impl DateFilter{
        //TODO: implement build method
    }

}

pub mod file{

    pub struct File{
        pub file_type: FileType,
        pub date: Date,
    }

    pub enum FileType{
        Image,
        Audio,
        Text,
    }

    pub struct Date{
        pub day: u8,
        pub month: u8,
        pub year: u32, 
    }
    
}