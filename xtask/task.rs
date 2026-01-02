use anyhow::bail;
use pinyinchch_type::{
    DagCharData, DagPhraseData, HmmData, HmmEmissionData, HmmPy2HzData, HmmTransitionData,
};
use rkyv::util::AlignedVec;
use std::fs::{File, create_dir_all, read_dir};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn convert_to_rkyv() -> anyhow::Result<()> {
    let data_path = PathBuf::from_str("data")?;
    if data_path.is_file() {
        bail!("The data cannot be a file");
    }
    for entry in read_dir(data_path)? {
        let path = entry?.path();
        if path.is_file() {
            to_rkyv(path)?;
        }
    }
    Ok(())
}

fn to_rkyv(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    let Some(file_name) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
        bail!("The path does not have a file name");
    };
    // rkyv 文件保存的目录
    let bin_data_root_path = PathBuf::from_str("bin_data")?;
    if bin_data_root_path.exists() && bin_data_root_path.is_file() {
        bail!("The bin_data cannot be a file");
    }
    if !bin_data_root_path.exists() {
        // 创建文件夹
        create_dir_all(&bin_data_root_path)?;
    }
    // rkyv 的文件名
    let new_file_name = file_name.replace(".json", ".rkyv");
    // 读取 json 和转换成 rkyv
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let bytes = to_bytes(reader, file_name)?;
    // 写入文件
    let file = File::create(bin_data_root_path.join(new_file_name))?;
    let mut writer = BufWriter::new(file);
    writer.write_all(bytes.as_slice())?;
    writer.flush()?;
    Ok(())
}

fn to_bytes(reader: BufReader<File>, file_name: &str) -> anyhow::Result<AlignedVec<16>> {
    let bytes = match file_name {
        "dag_char.json" => {
            let dag_char = serde_json::from_reader::<_, DagCharData>(reader)?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&dag_char)?
        }
        "dag_phrase.json" => {
            let dag_phrase = serde_json::from_reader::<_, DagPhraseData>(reader)?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&dag_phrase)?
        }
        "hmm_emission.json" => {
            let hmm_emission = serde_json::from_reader::<_, HmmEmissionData>(reader)?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_emission)?
        }
        "hmm_py2hz.json" => {
            let hmm_py2hz = serde_json::from_reader::<_, HmmPy2HzData>(reader)?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_py2hz)?
        }
        "hmm_start.json" => {
            let hmm_start = serde_json::from_reader::<_, HmmData>(reader)?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_start)?
        }
        "hmm_transition.json" => {
            let hmm_transition = serde_json::from_reader::<_, HmmTransitionData>(reader)?;
            rkyv::to_bytes::<rkyv::rancor::Error>(&hmm_transition)?
        }
        _ => bail!("This type of file conversion is not supported"),
    };

    Ok(bytes)
}
