const CACHE_NAME = 'v1::2::8';

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
  '/static/js/main.8718e994.chunk.js',
  '/static/js/main.chunk.js',
  '/static/js/bundle.js',
  '/static/js/0.chunk.js'
];

const URL_TO_BE_ROUTED_TO_INDEX = [
    '/about',
    '/profile',
    '/storage',
    '/products'
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
        .then(cacheNames => cacheNames.map(cacheName => {
            if(CACHE_NAME.indexOf(cacheName) === -1) {
                return caches.delete(cacheName);
            }
        }))
    );
});

this.addEventListener('fetch', e => {
    if(e.request.method === 'GET') {
        e.respondWith(
            caches.match(e.request)
            .then(response => {
                let url = new URL(e.request.url);
                if(URL_TO_BE_ROUTED_TO_INDEX.indexOf(url => url === url.pathname) !== -1) {
                    return caches.match(url.hostname);
                }
                return response || fetch(e.request).catch(notConnected);
            })
        );
    }
});

function notConnected() {
    return new Response('No internet connection', {status: 200, headers: {'Content-Type': 'text/html'}});
}