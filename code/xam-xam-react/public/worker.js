const CACHE_NAME = 'v1';
const RUNTIME = 'runtime';

const CACHED_URLS = [
  'index.html',
  './',
  '*.css',
  '*.js',
  'favicon.ico',
  'demo.js'
];

this.addEventListener('install',e => {
    e.waitUntil(
        caches.open(CACHE_NAME)
        .then(cache => cache.addAll(CACHED_URLS))
        .then(this.skipWaiting())
    );
});

this.addEventListener('activate', e => {
    const currentCaches = [CACHE_NAME,RUNTIME];
    e.waitUntil(
        caches.keys()
        .then(cacheNames => { 
            return cacheNames.filter(cacheName => !currentCaches.includes(cacheName))
        })
        .then(cachesToDelete => {
            return Promise.all(cachesToDelete.map(cacheToDelete => {
                return caches.delete(cacheToDelete);
            }));
        })
        .then(this.clients.claim())
    );
});

this.addEventListener('fetch', e => {
    if (e.request.url.startsWith(this.location.origin)) {
        e.respondWith(
            caches.match(e.request)
            .then(cachedResponse => {
                if(cachedResponse) {
                    return cachedResponse;
                }
                return caches.open(RUNTIME)
                .then(cache => {
                    return fetch(e.request)
                    .then(response => {
                        return cache.put(e.request, response.clone()).then(() => {
                            return response;
                        })
                    })
                })
            })
        );
    }
});