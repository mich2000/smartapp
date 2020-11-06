import React from 'react';
import ReactDOM from 'react-dom';
import UserContext from './components/user_context';
import {AppProvider} from './state';

ReactDOM.render(<AppProvider>
    <UserContext/>
</AppProvider>,document.getElementById('root'));