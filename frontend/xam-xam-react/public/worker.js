const CACHE_NAME = 'v1';

const CACHED_URLS = [
  '/favicon.ico',
  '/manifest.json',
  '/css/bootstrap.min.css',
  '/js/bootstrap.min.js',
  '/css/index.css',
  '/js/jquery-3.3.1.slim.min.js',
  '/css/modal.css',
  '/js/popper.min.js',
  '/images/logo-xamxam-512.png',
  '/',
  '/static/css/2.ccc6f86f.chunk.css',
  '/static/js/2.3d19b8e3.chunk.js',
  '/static/js/main.8718e994.chunk.js'
];

this.addEventListener('install',e => {
    e.waitUntil(
        caches.open(CACHE_NAME)
        .then(cache => cache.addAll(CACHED_URLS))
        .then(() => this.skipWaiting())
    );
});

this.addEventListener('activate', e => {
    e.waitUntil(
        caches.keys()
        .then(cacheNames => {
            cacheNames.map(cacheName => {
                if(cacheName !== CACHE_NAME) {
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
            console.log(e.request.url);
            return response || fetch(e.request).catch(notConnected);
        })
    );
});

function notConnected() {
    return new Response('No internet connection', {status: 200,headers: {'Content-Type': 'text/html'}});
}