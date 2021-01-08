import React from 'react';
import ReactDOM from 'react-dom';
import UserContext from './components/user_context';
import {AppProvider} from './state';
import {ToastContainer} from 'react-toastify';
import {showError, showInfo} from './toast';
import {Footer} from './footer';

if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('./worker.js', { scope: '.' });
}

const controlConnection = (_) => {
    if(navigator.onLine) {
        showInfo("You're back online.");
    } else if(!navigator.onLine) {
        showError("You're are offline");
    }
}

window.addEventListener('online', controlConnection);
window.addEventListener('offline', controlConnection);

ReactDOM.render(<AppProvider>
    <UserContext/>
    <ToastContainer/>
    <Footer/>
</AppProvider>,document.getElementById('root'));