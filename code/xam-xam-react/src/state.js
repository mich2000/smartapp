import React,{useState,createContext,useEffect} from 'react';
import api_functions from './api';

export const AppContext = createContext();

export const AppProvider = (props) => {
    const [user,setUser] = useState({
        email : '',
        loggedIn : false
    })

    useEffect(() => {
        let options = api_functions.method_get();
        fetch(api_functions.get_api() + "/auth/renew/token",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                setUser({...user,loggedIn : true});
                console.log("Token has been renewed.");
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    },[])

    return <AppContext.Provider value ={user,setUser}>
        {props.children}
    </AppContext.Provider>;
}