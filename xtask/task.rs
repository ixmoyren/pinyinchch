use pinyinchch_type::{
    DagChar, DagPhrase, HmmData, HmmEmissionData, HmmPy2HzData, HmmTransitionData,
};
use rkyv::util::AlignedVec;
use snafu::{Whatever, prelude::*};
use std::fs::{File, create_dir_all, read_dir};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn convert_to_rkyv() -> Result<(), Whatever> {
    let data_path =
        PathBuf::from_str("data").with_whatever_context(|_| "Couldn't crate path buf from str")?;
    if data_path.is_file() {
        whatever!("The data cannot be a file");
    }
    for entry in read_dir(data_path).with_whatever_context(|_| "Couldn't read data path")? {
        let path = entry
            .with_whatever_context(|_| "Couldn't get path from dir entry")?
            .path();
        if path.is_file() {
            to_rkyv(path)?;
        }
    }
    Ok(())
}

fn to_rkyv(path: impl AsRef<Path>) -> Result<(), Whatever> {
    let path = path.as_ref();
    let Some(file_name) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
        whatever!("The path does not have a file name");
    };
    // rkyv 文件保存的目录
    let bin_data_root_path = PathBuf::from_str("bin_data")
        .with_whatever_context(|_| "Couldn't crate path buf from str")?;
    if bin_data_root_path.exists() && bin_data_root_path.is_file() {
        whatever!("The bin_data cannot be a file");
    }
    if !bin_data_root_path.exists() {
        // 创建文件夹
        create_dir_all(&bin_data_root_path).with_whatever_context(|_| {
            format!("Couldn't create dir {}", bin_data_root_path.display())
        })?;
    }
    // rkyv 的文件名
    let new_file_name = file_name.replace(".json", ".rkyv");
    // 读取 json 和转换成 rkyv
    let file = File::open(path).with_whatever_context(|_| format!("Couldn't open {file_name}"))?;
    let reader = BufReader::new(file);
    let bytes = to_bytes(reader, file_name)?;
    // 写入文件
    let file = File::create(bin_data_root_path.join(&new_file_name))
        .with_whatever_context(|_| format!("Couldn't create {new_file_name}"))?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(bytes.as_slice())
        .with_whatever_context(|_| format!("Couldn't write to {new_file_name}"))?;
    Ok(())
}

fn to_bytes(reader: BufReader<File>, file_name: &str) -> Result<AlignedVec<16>, Whatever> {
    let bytes = match file_name {
        "dag_char.json" => {
            let dag_char = serde_json::from_reader::<_, DagChar>(reader)
                .with_whatever_context(|_| "Couldn't read dag_char.json")?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&dag_char)
                .with_whatever_context(|_| "The dag_char couldn't serialize to rkyv")?
        }
        "dag_phrase.json" => {
            let dag_phrase = serde_json::from_reader::<_, DagPhrase>(reader)
                .with_whatever_context(|_| "Couldn't read dag_phrase.json")?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&dag_phrase)
                .with_whatever_context(|_| "The dag_phrase couldn't serialize to rkyv")?
        }
        "hmm_emission.json" => {
            let hmm_emission = serde_json::from_reader::<_, HmmEmissionData>(reader)
                .with_whatever_context(|_| "Couldn't read hmm_emission.json")?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_emission)
                .with_whatever_context(|_| "The hmm_emission couldn't serialize to rkyv")?
        }
        "hmm_py2hz.json" => {
            let hmm_py2hz = serde_json::from_reader::<_, HmmPy2HzData>(reader)
                .with_whatever_context(|_| "Couldn't read hmm_py2hz.json")?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_py2hz)
                .with_whatever_context(|_| "The hmm_py2hz couldn't serialize to rkyv")?
        }
        "hmm_start.json" => {
            let hmm_start = serde_json::from_reader::<_, HmmData>(reader)
                .with_whatever_context(|_| "Couldn't read hmm_start.json")?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_start)
                .with_whatever_context(|_| "The hmm_start couldn't serialize to rkyv")?
        }
        "hmm_transition.json" => {
            let hmm_transition = serde_json::from_reader::<_, HmmTransitionData>(reader)
                .with_whatever_context(|_| "Couldn't read hmm_transition.json")?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_transition)
                .with_whatever_context(|_| "The hmm_transition couldn't serialize to rkyv")?
        }
        _ => whatever!("This type of file conversion is not supported"),
    };

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use crate::task::to_bytes;
    use pinyinchch_type::DagChar;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test() {
        let path = "../data/dag_char.json";
        // 读取 json 和转换成 rkyv
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let bytes = to_bytes(reader, "dag_char.json").unwrap();
        let _ =
            unsafe { rkyv::from_bytes_unchecked::<DagChar, rkyv::rancor::Error>(bytes.as_slice()) }
                .unwrap();
    }
}
