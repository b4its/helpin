export const useUpload = () => {
  const toDataURL = (file: File): Promise<string> =>
    new Promise((resolve, reject) => {
      const reader = new FileReader()
      reader.onload = () => resolve(reader.result as string)
      reader.onerror = reject
      reader.readAsDataURL(file)
    })

  // Upload gambar ke Nitro server route Nuxt → disimpan ke public/media
  const uploadImage = async (file: File): Promise<string> => {
    const data = await toDataURL(file)
    const res = await $fetch<{ url: string }>('/api/upload', {
      method: 'POST',
      body: { filename: file.name, data },
    })
    return res.url
  }

  return { uploadImage, toDataURL }
}
