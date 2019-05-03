import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import './registerServiceWorker'
import('pips-wasm')
  .then(pips => {
    (window as any)['roll'] = (input: string) => pips.roll(input);
    (window as any)['plot'] = (input: string) => pips.plot(input);
  })
  .catch(err => {
    console.error('error importing pips-wasm', err);
  });


Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
