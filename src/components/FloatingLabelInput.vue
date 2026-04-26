<script setup lang="ts">
// 定义 props
const props = defineProps<{
  label: string      // 标签文字，如 "账号"、"密码"
  type?: string      // 输入类型，默认 text
}>()

const modelValue = defineModel<string>({ default: "" })

const onInput = (event: Event) => {
  modelValue.value = (event.target as HTMLInputElement).value
}

// 将标签文字拆分成字符数组
const labelChars = props.label.split('')
</script>

<template>
  <div class="form-control">
    <input :type="type || 'text'" required
           :value="modelValue"
           @input="onInput"
    >
    <label>
      <span
          v-for="(char, index) in labelChars"
          :key="index"
          :style="{ transitionDelay: `${index * 60}ms` }"
      >
        {{ char }}
      </span>
    </label>
  </div>
</template>

<style scoped>
.form-control {
  position: relative;
  margin: 20px 0 40px;
  width: 100%;
}

.form-control input {
  background-color: transparent;
  border: 0;
  border-bottom: 2px #282936 solid;
  display: block;
  width: 100%;
  padding: 15px 0;
  font-size: 18px;
  color: #fff;
}

.form-control input:focus,
.form-control input:valid {
  outline: 0;
  border-bottom-color: #6d6da0;
}

.form-control label {
  position: absolute;
  top: 15px;
  left: 0;
  pointer-events: none;
}

.form-control label span {
  display: inline-block;
  font-size: 16px;
  font-weight: 500;
  min-width: 5px;
  color: #fff;
  transition: 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.form-control input:focus+label span,
.form-control input:valid+label span {
  color: #6d6da0;
  transform: translateY(-30px);
}
</style>
