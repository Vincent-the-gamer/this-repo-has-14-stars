import axios from "axios"

axios.post("http://localhost:8080/hook")
     .then(res => {
        console.log(res.data)
     })