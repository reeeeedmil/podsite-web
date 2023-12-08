window.sitemap = function () {
  const routes = router
    .getRoutes()
    .map(r => r.path)
    .filter(r => !r.includes(':')) // removes routes with params
    .map(r => `<url><loc>https://mysite${r}</loc></url>`)
  console.log(`
    <?xml version='1.0' encoding='UTF-8'?>
    <urlset xmlns='http://www.sitemaps.org/schemas/sitemap/0.9'>
        ${routes.join('\n')}
    </urlset>
  `)
}
