const re = /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\z.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;

const email_util = {
    control_email(email) {
        return re.test(email);
      }
}

export default email_util;