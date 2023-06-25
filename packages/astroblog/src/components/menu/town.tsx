import * as town from "@nxblog/town/dist/target/wasm/town/town"
import { useEffect } from 'react';

declare global {
  interface Window {
    go_town(s: string): void;
  }
}

export default () => {
  useEffect(() => {
    window.go_town = (s) => {
      console.log('sss', s);
      window.open("https://google.com", '_blank');
      // try {
      //   let audioContext = new window.AudioContext();
      //   audioContext.resume().then(() => {
      //   });
      // } catch (err) {
      //   console.error('Not in user interaction event loop:', err);
      // }
    }
    town.greet();
  }, []);
  return (
    <div>
    </div>
  );
}
