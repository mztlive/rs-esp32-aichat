// 360x360 屏幕布局常量和位置定义

/// 屏幕尺寸常量
pub const SCREEN_WIDTH: i32 = 360;
pub const SCREEN_HEIGHT: i32 = 360;

/// 屏幕中心点
pub const SCREEN_CENTER_X: i32 = SCREEN_WIDTH / 2; // 180
pub const SCREEN_CENTER_Y: i32 = SCREEN_HEIGHT / 2; // 180

/// 四个角的坐标
pub const TOP_LEFT: (i32, i32) = (0, 0);
pub const TOP_RIGHT: (i32, i32) = (SCREEN_WIDTH - 1, 0);
pub const BOTTOM_LEFT: (i32, i32) = (0, SCREEN_HEIGHT - 1);
pub const BOTTOM_RIGHT: (i32, i32) = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1);

/// 边缘中心点
pub const TOP_CENTER: (i32, i32) = (SCREEN_CENTER_X, 0);
pub const BOTTOM_CENTER: (i32, i32) = (SCREEN_CENTER_X, SCREEN_HEIGHT - 1);
pub const LEFT_CENTER: (i32, i32) = (0, SCREEN_CENTER_Y);
pub const RIGHT_CENTER: (i32, i32) = (SCREEN_WIDTH - 1, SCREEN_CENTER_Y);

/// 九宫格布局坐标 (每个区域120x120像素)
pub const GRID_SIZE: i32 = 120;

// 九宫格左上角坐标
pub const GRID_TOP_LEFT: (i32, i32) = (0, 0);
pub const GRID_TOP_CENTER: (i32, i32) = (GRID_SIZE, 0);
pub const GRID_TOP_RIGHT: (i32, i32) = (GRID_SIZE * 2, 0);
pub const GRID_MIDDLE_LEFT: (i32, i32) = (0, GRID_SIZE);
pub const GRID_MIDDLE_CENTER: (i32, i32) = (GRID_SIZE, GRID_SIZE);
pub const GRID_MIDDLE_RIGHT: (i32, i32) = (GRID_SIZE * 2, GRID_SIZE);
pub const GRID_BOTTOM_LEFT: (i32, i32) = (0, GRID_SIZE * 2);
pub const GRID_BOTTOM_CENTER: (i32, i32) = (GRID_SIZE, GRID_SIZE * 2);
pub const GRID_BOTTOM_RIGHT: (i32, i32) = (GRID_SIZE * 2, GRID_SIZE * 2);

// 九宫格中心点坐标
pub const GRID_CENTER_TOP_LEFT: (i32, i32) = (GRID_SIZE / 2, GRID_SIZE / 2);
pub const GRID_CENTER_TOP_CENTER: (i32, i32) = (GRID_SIZE + GRID_SIZE / 2, GRID_SIZE / 2);
pub const GRID_CENTER_TOP_RIGHT: (i32, i32) = (GRID_SIZE * 2 + GRID_SIZE / 2, GRID_SIZE / 2);
pub const GRID_CENTER_MIDDLE_LEFT: (i32, i32) = (GRID_SIZE / 2, GRID_SIZE + GRID_SIZE / 2);
pub const GRID_CENTER_MIDDLE_CENTER: (i32, i32) =
    (GRID_SIZE + GRID_SIZE / 2, GRID_SIZE + GRID_SIZE / 2);
pub const GRID_CENTER_MIDDLE_RIGHT: (i32, i32) =
    (GRID_SIZE * 2 + GRID_SIZE / 2, GRID_SIZE + GRID_SIZE / 2);
pub const GRID_CENTER_BOTTOM_LEFT: (i32, i32) = (GRID_SIZE / 2, GRID_SIZE * 2 + GRID_SIZE / 2);
pub const GRID_CENTER_BOTTOM_CENTER: (i32, i32) =
    (GRID_SIZE + GRID_SIZE / 2, GRID_SIZE * 2 + GRID_SIZE / 2);
pub const GRID_CENTER_BOTTOM_RIGHT: (i32, i32) =
    (GRID_SIZE * 2 + GRID_SIZE / 2, GRID_SIZE * 2 + GRID_SIZE / 2);

/// 常用边距
pub const MARGIN_SMALL: i32 = 10;
pub const MARGIN_MEDIUM: i32 = 20;
pub const MARGIN_LARGE: i32 = 30;

/// 内容区域（带边距）
pub const CONTENT_AREA_START_X: i32 = MARGIN_MEDIUM;
pub const CONTENT_AREA_START_Y: i32 = MARGIN_MEDIUM;
pub const CONTENT_AREA_END_X: i32 = SCREEN_WIDTH - MARGIN_MEDIUM;
pub const CONTENT_AREA_END_Y: i32 = SCREEN_HEIGHT - MARGIN_MEDIUM;
pub const CONTENT_AREA_WIDTH: i32 = CONTENT_AREA_END_X - CONTENT_AREA_START_X;
pub const CONTENT_AREA_HEIGHT: i32 = CONTENT_AREA_END_Y - CONTENT_AREA_START_Y;

/// 圆形区域相关常量
pub const CIRCLE_RADIUS_SMALL: i32 = 20;
pub const CIRCLE_RADIUS_MEDIUM: i32 = 40;
pub const CIRCLE_RADIUS_LARGE: i32 = 60;
pub const CIRCLE_RADIUS_EXTRA_LARGE: i32 = 80;

/// 文字相关常量
pub const TEXT_LINE_HEIGHT: i32 = 22; // 基于10x20字体
pub const TEXT_CHAR_WIDTH: i32 = 10;

/// 九宫格位置枚举
#[derive(Debug, Clone, Copy)]
pub enum GridPosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl GridPosition {
    /// 获取九宫格位置的左上角坐标
    pub fn get_top_left(&self) -> (i32, i32) {
        match self {
            GridPosition::TopLeft => GRID_TOP_LEFT,
            GridPosition::TopCenter => GRID_TOP_CENTER,
            GridPosition::TopRight => GRID_TOP_RIGHT,
            GridPosition::MiddleLeft => GRID_MIDDLE_LEFT,
            GridPosition::MiddleCenter => GRID_MIDDLE_CENTER,
            GridPosition::MiddleRight => GRID_MIDDLE_RIGHT,
            GridPosition::BottomLeft => GRID_BOTTOM_LEFT,
            GridPosition::BottomCenter => GRID_BOTTOM_CENTER,
            GridPosition::BottomRight => GRID_BOTTOM_RIGHT,
        }
    }

    /// 获取九宫格位置的中心点坐标
    pub fn get_center(&self) -> (i32, i32) {
        match self {
            GridPosition::TopLeft => GRID_CENTER_TOP_LEFT,
            GridPosition::TopCenter => GRID_CENTER_TOP_CENTER,
            GridPosition::TopRight => GRID_CENTER_TOP_RIGHT,
            GridPosition::MiddleLeft => GRID_CENTER_MIDDLE_LEFT,
            GridPosition::MiddleCenter => GRID_CENTER_MIDDLE_CENTER,
            GridPosition::MiddleRight => GRID_CENTER_MIDDLE_RIGHT,
            GridPosition::BottomLeft => GRID_CENTER_BOTTOM_LEFT,
            GridPosition::BottomCenter => GRID_CENTER_BOTTOM_CENTER,
            GridPosition::BottomRight => GRID_CENTER_BOTTOM_RIGHT,
        }
    }
}

/// 屏幕区域定义
#[derive(Debug, Clone, Copy)]
pub struct ScreenRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl ScreenRect {
    /// 创建新的屏幕矩形区域
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// 获取中心点坐标
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// 获取右下角坐标
    pub fn bottom_right(&self) -> (i32, i32) {
        (self.x + self.width - 1, self.y + self.height - 1)
    }
}

/// 预定义的屏幕区域
pub const FULL_SCREEN: ScreenRect = ScreenRect {
    x: 0,
    y: 0,
    width: SCREEN_WIDTH,
    height: SCREEN_HEIGHT,
};
pub const CONTENT_AREA: ScreenRect = ScreenRect {
    x: CONTENT_AREA_START_X,
    y: CONTENT_AREA_START_Y,
    width: CONTENT_AREA_WIDTH,
    height: CONTENT_AREA_HEIGHT,
};

/// 顶部状态栏区域
pub const STATUS_BAR: ScreenRect = ScreenRect {
    x: 0,
    y: 0,
    width: SCREEN_WIDTH,
    height: 30,
};

/// 底部操作栏区域
pub const ACTION_BAR: ScreenRect = ScreenRect {
    x: 0,
    y: SCREEN_HEIGHT - 30,
    width: SCREEN_WIDTH,
    height: 30,
};

/// 顶部状态栏文字区域
pub const STATUS_BAR_TEXT: ScreenRect = ScreenRect {
    x: 10,
    y: 10,
    width: 100,
    height: 20,
};
