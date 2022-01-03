macro_rules! json_travel {
	($jsn:expr,$x:expr) => (&$jsn[$x]);

    ($jsn:expr,$x:expr,$($y:expr),*) => (
       json_travel!($jsn[$x],$($y),*)
    )
}

macro_rules! vid_travel {
	($jsn:expr,$($x:expr),*) => {
		json_travel!($jsn,$($x),*).as_str().unwrap_or("").to_string()
	};
}
#[macro_export]
macro_rules! iter_collect {
    ($vector:expr,$func:expr) => {
        $vector.iter().map($func).collect()
    };
    (into $vector:expr,$func:expr) => {
        $vector.into_iter().map($func).collect()
    };
}

macro_rules! json_file {
    (write $x:expr,$fn:expr) => {
        let j = serde_json::to_string($x).expect("Coudnot serialize");
        let mut fp = std::fs::File::create($fn).expect("could not create File");
        fp.write(j.as_bytes()).expect("failed to write to file");
    };
    (read $t:ty,$x:expr) => {
        let j = fs::read_to_string($x).expect("");
        let j: $t = serde_json::from_str(&j).expect("");
        j
    };
}
macro_rules! static_format {
	($($arg:tt) *) => {
        unsafe{
		format!($($arg) *)
        }
	};
}

pub(crate) use iter_collect;
pub(crate) use json_file;
pub(crate) use json_travel;
pub(crate) use static_format;
pub(crate) use vid_travel;
