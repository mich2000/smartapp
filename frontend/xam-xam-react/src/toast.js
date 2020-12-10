import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

let options = {
    position: "top-center",
    autoClose: 5000,
    hideProgressBar: false,
    closeOnClick: true,
    pauseOnHover: false,
    draggable: false,
    progress: undefined,
};

export const show_error = (msg) => {
    toast.error(msg, options);
};

export const show_info = (msg) => {
    toast.info(msg,options);
}