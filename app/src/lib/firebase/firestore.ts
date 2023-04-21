import { firestore } from './init'
import { Bytes, collection, doc, setDoc, addDoc, getDocs } from 'firebase/firestore'

const MODELS = "models"
const PATCHES = "patches"

export const patches = async (id: string): Promise<Array<Uint8Array>> => {
  let ref = collection(firestore, MODELS, id, "patches")
  let docs = await getDocs(ref)
  return docs.docs.map((patch) => {
    let data = patch.data().data as Bytes
    return data.toUint8Array()
  })
}

export const appendPatch = async (id: string, patch: Uint8Array) => {
  await setDoc(doc(firestore, MODELS, id), {id}, {merge: true})
  await addDoc(collection(firestore, MODELS, id, PATCHES), {data: Bytes.fromUint8Array(patch)})
}
