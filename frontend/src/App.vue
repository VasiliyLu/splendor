<template>
  <div class="container">
    <h1>Splendor Vision</h1>
    
    <div class="video-container">
      <video ref="video" autoplay playsinline muted></video>
      <canvas ref="canvas" style="display: none;"></canvas>
      
      <div v-if="status" class="status" :class="status.type">
        {{ status.message }}
      </div>
    </div>

    <div class="controls">
      <button @click="toggleCamera" :disabled="isLoading">
        {{ isStreaming ? 'Остановить' : 'Запустить камеру' }}
      </button>
    </div>

    <div v-if="results" class="results">
      <h3>Результаты:</h3>
      <pre>{{ JSON.stringify(results, null, 2) }}</pre>
    </div>
  </div>
</template>

<script setup>
import {onUnmounted, ref} from 'vue';

const video = ref(null);
const canvas = ref(null);
const results = ref(null);
const status = ref({ type: 'info', message: 'Готов к работе' });
const isStreaming = ref(false);
const isLoading = ref(false);

let socket = null;
let intervalId = null;

const connectWebSocket = () => {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const host = window.location.host;
  // Используем относительный путь для проксирования или прямого подключения
  const wsUrl = `${protocol}//${host}/ws/vision`;
  
  socket = new WebSocket(wsUrl);
  
  socket.onopen = () => {
    status.value = { type: 'success', message: 'WebSocket подключен' };
  };
  
  socket.onmessage = (event) => {
    try {
      results.value = JSON.parse(event.data);
    } catch (e) {
      console.error('Ошибка парсинга ответа:', e);
    }
  };
  
  socket.onclose = () => {
    status.value = { type: 'error', message: 'WebSocket закрыт' };
    stopStreaming();
  };
  
  socket.onerror = (error) => {
    console.error('WebSocket Error:', error);
    status.value = { type: 'error', message: 'Ошибка WebSocket' };
  };
};

const sendFrame = () => {
  if (!socket || socket.readyState !== WebSocket.OPEN || !isStreaming.value) return;
  
  const v = video.value;
  const c = canvas.value;
  if (!v || !c) return;

  // Устанавливаем размер canvas под видео
  if (c.width !== v.videoWidth || c.height !== v.videoHeight) {
    c.width = v.videoWidth;
    c.height = v.videoHeight;
  }

  const ctx = c.getContext('2d');
  ctx.drawImage(v, 0, 0, c.width, c.height);
  
  // Конвертируем в Blob (JPEG для экономии трафика)
  c.toBlob((blob) => {
    if (blob && socket.readyState === WebSocket.OPEN) {
      blob.arrayBuffer().then(buffer => {
        socket.send(buffer);
      });
    }
  }, 'image/jpeg', 0.8);
};

const startStreaming = async () => {
  isLoading.value = true;
  try {
    video.value.srcObject = await navigator.mediaDevices.getUserMedia({
      video: {facingMode: 'environment'}
    });
    isStreaming.value = true;
    
    connectWebSocket();
    
    // Отправляем кадры примерно 5 раз в секунду
    intervalId = setInterval(sendFrame, 200);
    
    status.value = { type: 'success', message: 'Трансляция запущена' };
  } catch (err) {
    console.error('Ошибка доступа к камере:', err);
    status.value = { type: 'error', message: 'Нет доступа к камере' };
  } finally {
    isLoading.value = false;
  }
};

const stopStreaming = () => {
  isStreaming.value = false;
  
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
  
  if (video.value && video.value.srcObject) {
    video.value.srcObject.getTracks().forEach(track => track.stop());
    video.value.srcObject = null;
  }
  
  if (socket) {
    socket.close();
    socket = null;
  }
  
  status.value = { type: 'info', message: 'Трансляция остановлена' };
};

const toggleCamera = () => {
  if (isStreaming.value) {
    stopStreaming();
  } else {
    startStreaming();
  }
};

onUnmounted(() => {
  stopStreaming();
});
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  font-family: sans-serif;
  text-align: center;
}

.video-container {
  position: relative;
  background: #000;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 20px;
  aspect-ratio: 16 / 9;
}

video {
  width: 100%;
  height: 100%;
  display: block;
}

.status {
  position: absolute;
  top: 10px;
  left: 10px;
  padding: 5px 10px;
  border-radius: 4px;
  color: white;
  font-size: 14px;
  background: rgba(0,0,0,0.5);
}

//.status.success { background: rgba(40, 167, 69, 0.8); }
//.status.error { background: rgba(220, 53, 69, 0.8); }
//.status.info { background: rgba(0, 123, 255, 0.8); }

.controls {
  margin-bottom: 20px;
}

button {
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
  background: #42b883;
  color: white;
  border: none;
  border-radius: 4px;
}

button:disabled {
  background: #ccc;
}

.results {
  text-align: left;
  background: #f4f4f4;
  padding: 15px;
  border-radius: 8px;
}

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>
