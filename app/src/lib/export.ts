import FileSaver from 'file-saver';

export const exportJson = (filename: string, content: string) => {
  const blob = new Blob([content], { type: "application/json;charset=utf-8" });
  FileSaver.saveAs(blob, `${filename}.json`, { autoBom: true });
};
