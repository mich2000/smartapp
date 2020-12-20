const CACHE_NAME = 'v1';
const RUNTIME = 'runtime';

const CACHED_URLS = [
  'index.html',
  '*.css',
  '*.js',
  '*.img',
  'favicon.ico',
  'manifest.json',
  '/'
];

this.addEventListener('install',e => {
    e.waitUntil(
        caches.open(CACHE_NAME)
        .then(cache => cache.addAll(CACHED_URLS))
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
    );
});

this.addEventListener('fetch', e => {
        e.respondWith(
                caches.match(e.request)
                .then(response => {
                        if(response) {
                                return response;
                        }
                        fetch(e.request)
                        .then(response => {
                                return response;
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