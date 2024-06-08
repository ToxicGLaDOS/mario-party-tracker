<script setup lang="ts">
  var form: HTMLFormElement;
  var span: HTMLElement;


  // Direct complaints here:
  // https://stackoverflow.com/a/37856924/3697905
  function getCookies (cookieStr: string) {
    return cookieStr.split(";")
      .map(str => str.trim().split(/=(.+)/))
      .reduce((acc: Record<string, string>, curr: string[]) => {
          acc[curr[0]] = curr[1];
          return acc;
      }, {})
  }

  console.log(getCookies(document.cookie));

  window.onload = function () {
    form = document.querySelector("#login_info") as HTMLFormElement;
    span = document.querySelector("#flash") as HTMLSpanElement;

    // Take over form submission
    form.addEventListener("submit", (event) => {
      event.preventDefault();
      sendData();
    });
  }

  async function sendData() {
    // Associate the FormData object with the form element
    const formData = new FormData(form);
    // By default FormData is a multipart form, but we want a normal
    // application/x-www-form-urlencoded form, so we convert it here
    const urlEncodedForm = new URLSearchParams(formData as any).toString();

    var response = await fetch("/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: urlEncodedForm
    })

    if (response.ok) {
      window.location.href = "/input";
    }
    else {
      // Show flash
      span.innerHTML = "Failed to log in";
    }
  }
</script>

<template>
  <main>
    <form id="login_info" action="/login" method="post">
      <div class="vertical-align">
        <h1 class="title">Login</h1>
        <span id="flash"></span>
        <div>
          <input id="username" name="username" placeholder="Username"/>
          <input id="password" name="password" placeholder="Password"/>
        </div>
        <input class="submit" type="submit" value="Login">
      </div>
    </form>
  </main>
</template>

<style scoped>
  main {
    display: flex;
    min-width: 0;
  }
  .title {
    font-size: 75px;
    align-self: center;
  }
  .vertical-align {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
  .submit {
    align-self: center;
  }
</style>
