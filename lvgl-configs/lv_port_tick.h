/* lv_port_tick.h — 由 LV_TICK_CUSTOM_INCLUDE 引入 */
#pragma once
#include "esp_timer.h"
static inline uint32_t lv_port_tick_get(void) {
  return esp_timer_get_time() / 1000; /* μs → ms */
}