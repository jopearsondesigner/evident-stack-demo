import { supabase } from '$lib/supabase/client'

const BUCKET = 'interface-images'

// HT: https://github.com/DimitarNestorov/file-to-hash/blob/master/packages/blob-to-hash/src/index.ts, adapted
const blobSha = (blob: Blob): Promise<string> => {
  return new Promise((resolve, reject) => {
    const fileReader = new FileReader()

    fileReader.addEventListener('load', () => {
      crypto.subtle.digest('SHA-1', fileReader.result as ArrayBuffer).then((buffer) => {
        resolve(btoa(String.fromCharCode(...new Uint8Array(buffer))))
      })
    })
    fileReader.addEventListener('error', () => {
      reject(fileReader.error)
    })

    console.log("blob", blob)
    fileReader.readAsArrayBuffer(blob)
  })
}

export const upload_interface_image = async (
  model_id: string,
  blob: Blob
) => {
  let sha = await blobSha(blob)
  console.log("upload_interface_image", model_id, blob, sha)
  let name = `${model_id}/${sha}`
  await supabase.storage.from(BUCKET).upload(name, blob, { upsert: true })
  const { data } = supabase.storage.from(BUCKET).getPublicUrl(name)
  console.log("upload_interface_image returning:", data)
  return data.publicUrl
}
