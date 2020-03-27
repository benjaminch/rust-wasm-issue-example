# Goal
This repository is made to illustrate the issue raised in this post: https://stackoverflow.com/questions/60882176/rust-wasm-function-string-args-blank-when-calling-async-rust-function-from-jav

# Run
```
docker build -t wam_issue_example .
docker run -d -p 80:80 wam_issue_example
```

# Reproducing
Open your browser and go to `localhost:80`, open the JS console, click on the button and observe that the params passed as args in the function:

```
backend_api_domain=

token=

/weddings//guests/registration/AAAA
```