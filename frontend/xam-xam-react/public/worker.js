const CACHE_NAME = 'v1::3::9';

this.addEventListener('install',e => {
    e.waitUntil(
        caches.open(CACHE_NAME)
        .then(cache => cache.addAll([
            '/favicon.ico',
            '/manifest.json',
            '/',
            '/images/logo-xamxam-512.png',
            '/images/logo-xamxam-114.png',
          ]))
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
        const url_response = new URL(e.request.url);
        e.respondWith(
            caches.open(CACHE_NAME)
            .then(cache => cache.match(e.request))
            .then(response => {
                if([ '/about', '/profile', '/storage' ].some(url => url_response.pathname.endsWith(url))) {
                    return caches.match('/');
                }
                return response || fetch(e.request)
                .then(re => {
                    if([ '/static', '/js', '/css' ].some(ext => url_response.pathname.indexOf(ext) !== -1)) {
                        let copy = re.clone();
                        caches.open(CACHE_NAME).then(cache => cache.put(url_response.pathname,copy));
                    }
                    return re;
                }).catch(notConnected);
            })
        );
    }
});

function notConnected() {
    return new Response(
        'No internet connection',
        {
            status: 200,
            headers: {
                'Content-Type': 'text/html'
            }
        }
    );
}