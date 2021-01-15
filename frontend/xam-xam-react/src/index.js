import React from 'react';
import ReactDOM from 'react-dom';
import UserContext from './components/user_context';
import {AppProvider} from './state';
import {ToastContainer} from 'react-toastify';
import {showError, showInfo} from './toast';
import {Footer} from './footer';
import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/js/dist/button';
import 'bootstrap/js/dist/collapse';
import 'bootstrap/js/dist/dropdown';
import 'jquery';
import 'popper.js';
import './css/index.css';
import './css/modal.css';

if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('worker.js')
    .then(registration => registration.update())
    .then(() => console.log('Service worker has been registered and updated.'));
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