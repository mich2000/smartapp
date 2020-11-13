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

    useEffect(() => {
        fetch(api_functions.get_api() + "/auth/email",api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then(json => {
                    setUser({email : json.email, loggedIn : true});
                })
                .catch((e) => console.error(`Could not send through the request. error: ${e}`));
            } else {
                api_call.text()
                .then((body) => console.log(body))
                .catch((e) => console.error(`Could not send through the request. error: ${e}`));
            }
        })
        .catch((e) => console.error(`Could not send through the request. error: ${e}`));
    },[])

    return <AppContext.Provider value ={[user,setUser]}>
        {props.children}
    </AppContext.Provider>;
}