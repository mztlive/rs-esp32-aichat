/* ── AFE（含 AEC / VAD / NS） ─────────────────────── */
#include "esp_afe_sr_iface.h"
#include "esp_afe_sr_models.h"   // ⭐ 若你用自己的 AFE 模型，可删

/* ── WakeNet & MultiNet ─────────────────────────── */
#include "esp_wn_iface.h"
#include "esp_wn_models.h"       // ⭐ 自定义模型的话也可删
#include "esp_mn_iface.h"
#include "esp_mn_models.h"

/* ── 可选扩展：纯 AEC、DOA、NSN ───────────────────── */
#include "esp_afe_aec.h"         // 只做 AFE-AEC 对比时用
#include "esp_doa.h"             // 需要声源角度时用
#include "esp_nsn_iface.h"
#include "esp_nsn_models.h"
