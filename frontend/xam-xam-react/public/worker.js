const CACHE_NAME = 'v1';

const CACHED_URLS = [
  '/index.html',
  '/favicon.ico',
  '/manifest.json',
  '/'
];

this.addEventListener('install',e => {
    e.waitUntil(
        caches.open(CACHE_NAME)
        .then(cache => cache.addAll(CACHED_URLS))
    );
});

this.addEventListener('activate', e => {
    e.waitUntil(
        caches.keys()
        .then(cacheNames => {
            cacheNames.map(cacheName => {
                if(cacheName.indexOf(CACHE_NAME) < 0) {
                    return caches.delete(cacheName);
                }
            });
        })
    );
});

this.addEventListener('fetch', e => {
    e.respondWith(
        caches.match(e.request)
        .then(response => {
            return response || fetch(e.request)
            .then(fetchResponse => {
                return fetchResponse
            })
            .catch(notConnected);
        })
    );
});

//status code => important http status
function notConnected() {
    return new Response('No internet connection', {
        status: 200,
        headers: {
            'Content-Type': 'text/html'
        }
    });
}