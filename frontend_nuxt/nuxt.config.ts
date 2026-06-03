// nuxt.config.ts
export default defineNuxtConfig({
  app: {
    head: {
      htmlAttrs: { lang: 'id' },
      link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap' },
        { rel: 'stylesheet', href: 'https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css' }
      ],
      script: [
        // Ubah body: true menjadi tagPosition: 'bodyClose'
        { src: '/script.js', tagPosition: 'bodyClose' } 
      ],
      base: { target: '_blank' }
    }
  }
})