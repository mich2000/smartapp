import React from 'react';
import ReactDOM from 'react-dom';
import UserContext from './components/user_context';
import {AppProvider,AppConsumer} from './state';

if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('./worker.js');
}

ReactDOM.render(<AppProvider>
    <UserContext/>
</AppProvider>,document.getElementById('root'));