/**
 * lv_conf.h — 精简版 for ESP32-S3 + ST7796 480×320
 * 只保留最常用宏，其他保持默认
 */

/*----------------
 * 基本参数
 *--------------*/
#define LV_HOR_RES_MAX 480 /* 屏幕宽 */
#define LV_VER_RES_MAX 320 /* 屏幕高 */
#define LV_COLOR_DEPTH 16  /* ST7796 用 16-bit RGB565 */

#define LV_TICK_CUSTOM 1 /* 用你的 esp_timer/FreeRTOS 1 ms 定时 */
#if LV_TICK_CUSTOM
extern uint32_t lv_port_tick_get(void);
#define LV_TICK_CUSTOM_INCLUDE "lv_port_tick.h" /* 里面声明上面的函数 */
#define LV_TICK_CUSTOM_SYS_TIME_EXPR (lv_port_tick_get())
#endif

/*----------------
 * 内存配置
 *--------------*/
#define LV_MEM_SIZE (64U * 1024U) /* 64 KB 内部堆给 LVGL */
#define LV_MEM_POOL_INCLUDE <esp_heap_caps.h>
#define LV_MEM_CUSTOM 1 /* 把大块放到 PSRAM */
#if LV_MEM_CUSTOM
#define LV_MEM_CUSTOM_ALLOC heap_caps_malloc
#define LV_MEM_CUSTOM_FREE heap_caps_free
#define LV_MEM_CUSTOM_REALLOC heap_caps_realloc
#define LV_MEM_CUSTOM_INCLUDE <esp_heap_caps.h>
#endif

/*----------------
 * 显示与输入驱动
 *--------------*/
#define LV_DISPLAY_RENDER_START_CB 1 /* 刷屏前回调（可开 DMA） */

/*----------------
 * 选用组件
 *--------------*/
#define LV_USE_LOG 0      /* 不要在 release 下打印 */
#define LV_USE_ANIMIMG 1  /* gif/png 序列帧 */
#define LV_USE_FS_STDIO 0 /* 如果走 FATFS/SD 卡，再另行开启 */

/*----------------
 * 中文/多语言字体
 *--------------*/
#define LV_FONT_MONTSERRAT_16 1
#define LV_FONT_UNSCII_16 0
/* 如要导入 TTF/中文点阵，可用 font_conv 生成 C 数组再额外 include */

#endif /* LV_CONF_H */