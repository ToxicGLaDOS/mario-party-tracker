<script setup lang="ts">
</script>

<template>
  <main>
    <form action="http://localhost:8081/login">
      <div class="vertical-align">
        <h1 class="title">Login</h1>
        <div>
          <input id="username" placeholder="Username"/>
          <input id="password" placeholder="Password"/>
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
