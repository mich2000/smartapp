let api = process.env.REACT_APP_ID_API_URL || 'https://localhost:8080';
let business_api = process.env.REACT_APP_BIS_API_URL || 'https://localhost:8081';

//basic options for every api fetch call
let basicOptions = function() {
    return {
        mode: "cors", // no-cors, *cors, same-origin
        cache: "no-store", // *default, no-cache, reload, force-cache, only-if-cached
        credentials: "include", // include, *same-origin, omit
        headers: {
            "Content-Type": "application/json",
            "Accept": "application/json",
        }
    };
}

const api_functions = {
    get_api() {
        return api.toString();
    },
    get_business_api() {
        return business_api.toString();
    },
    method_get() {
        return basicOptions();
    },
    method_delete() {
        let opties = basicOptions();
        opties.method = "DELETE";
        return opties;
    },
    method_post() {
        let opties = basicOptions();
        opties.method = "POST";
        return opties;
    },
    method_put() {
        let opties = basicOptions();
        opties.method = "PUT";
        return opties;
    }
}

export default api_functions;