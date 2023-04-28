import { firestore } from './init'

type PatchDoc = {fields: {data: {bytesValue: string}}}

export const patches = async (id: string, user: string) => {
  // const request = await firestore()

  // // TODO: get merged attribute of model and all unmerged patches from collection

  // let patches: Array<string> = []
  // let url = request.patches_url(id)
  // let response: Response;
  // let next_page_token: string | undefined = undefined;

  // do {
  //   response = await fetch(url, {headers: request.headers})
  //   let body: {documents: Array<PatchDoc>, nextPageToken: string} = await response.json()

  //   body.documents.forEach((doc) => {
  //     patches.push(doc.fields.data.bytesValue)
  //   });

  //   if (body.nextPageToken) {
  //     next_page_token = body.nextPageToken
  //     url = request.patches_url(id) + new URLSearchParams({
  //       'nextPageToken': next_page_token,
  //     })
  //   }
  // } while (next_page_token)

  // return patches
}

export const addPatch = async (id: string, user: string, data: string) => {
  // const request = await firestore()

  // let response = await fetch(request.patches_url(id), {
  //   method: 'POST',
  //   headers: request.headers,
  //   body: JSON.stringify({
  //     user,
  //     data: {bytesValue: data},
  //     merged: false
  //   })
  // })

  console.log("Firestore config", request, response)
}
