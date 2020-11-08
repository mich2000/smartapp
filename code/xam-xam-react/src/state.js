import React,{useState,createContext,useEffect} from 'react';
import api_functions from './api';

export const AppContext = createContext({
    email : '',
    loggedIn : false
});

export const AppProvider = (props) => {
    const [user,setUser] = useState({
        email : '',
        loggedIn : false
    });

    return <AppContext.Provider value ={[user,setUser]}>
        {props.children}
    </AppContext.Provider>;
}