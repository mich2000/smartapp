import React,{useState,createContext} from 'react';

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