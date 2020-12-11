import React from 'react';
import ReactDOM from 'react-dom';
import UserContext from './components/user_context';
import {AppProvider} from './state';
import {ToastContainer} from 'react-toastify';
import {show_info,show_error} from './toast';

if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('./worker.js');
}

ReactDOM.render(<AppProvider>
    <UserContext/>
    <ToastContainer/>
</AppProvider>,document.getElementById('root'));