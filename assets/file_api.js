export async function open_config_file_str() {
  const options = {
    types: [
      {
        description: "Text Files",
        accept: {
          "text/plain": [".txt"],
        },
      },
    ],
  };

  const [fileHandle] = await window.showOpenFilePicker(options);
  const file = await fileHandle.getFile();
  const contents = await file.text();

  return {
    file_name: file.name,
    config_str: contents,
  };
}

export async function save_config_file_str(file_name, config_str) {
  const cfg_file_blob = new Blob([config_str], { type: "text/plain" });

  const pickerOptions = {
    suggestedName: file_name,
    types: [
      {
        description: "Abeeway Configuration File",
        accept: {
          "text/plain": [".txt"],
        },
      },
    ],
  };

  const fileHandle = await window.showSaveFilePicker(pickerOptions);
  const fileName = (await fileHandle.getFile()).name;
  const writableFileStream = await fileHandle.createWritable();
  await writableFileStream.write(cfg_file_blob);
  await writableFileStream.close();

  return fileName;
}
