-- 创建数据库
CREATE DATABASE RiseStar;
USE RiseStar;

-- 创建用户表
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY COMMENT '用户ID',
    nickname VARCHAR(50) NOT NULL COMMENT '用户昵称',
    bio TEXT COMMENT '个性签名',
    password VARCHAR(255) NOT NULL COMMENT '密码',
    total_passed_questions INT DEFAULT 0 COMMENT '通过的题目总数',
    open_id VARCHAR(100) NOT NULL UNIQUE COMMENT '微信账号唯一绑定的open_id',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 创建管理员表
CREATE TABLE admins (
    id INT AUTO_INCREMENT COMMENT '管理员ID',
    nickname VARCHAR(50) NOT NULL COMMENT '用户昵称',
    password VARCHAR(255) NOT NULL COMMENT '密码',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

-- 创建分类表
CREATE TABLE categories (
    id INT AUTO_INCREMENT COMMENT '分类ID',
    name VARCHAR(100) NOT NULL COMMENT '分类名称',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

-- 创建标签表
CREATE TABLE tags (
    id INT AUTO_INCREMENT COMMENT '标签ID',
    category_id INT NOT NULL COMMENT '所属分类ID',
    name VARCHAR(50) NOT NULL COMMENT '标签名称',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- 创建题目表
CREATE TABLE questions (
    id INT AUTO_INCREMENT COMMENT '题目ID',
    category_id INT NOT NULL COMMENT '所属分类ID',
    description TEXT NOT NULL COMMENT '题目描述',
    answer TEXT NOT NULL COMMENT '题目答案',
    question_type ENUM('text', 'single_select', 'multiple_select') NOT NULL COMMENT '题目类型',
    passed_count INT DEFAULT 0 COMMENT '通过次数',
    user_attempt_count INT DEFAULT 0 COMMENT '用户尝试次数',
    total_attempt_count INT DEFAULT 0 COMMENT '总尝试次数',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- 创建题目标签关联表
CREATE TABLE question_tags (
    question_id INT NOT NULL COMMENT '题目ID',
    tag_id INT NOT NULL COMMENT '标签ID',
    PRIMARY KEY (question_id, tag_id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

-- 创建用户题目关联表（记录用户是否通过题目）
CREATE TABLE user_questions (
    user_id INT NOT NULL COMMENT '用户ID',
    question_id INT NOT NULL COMMENT '题目ID',
    is_passed BOOLEAN DEFAULT FALSE COMMENT '是否通过',
    PRIMARY KEY (user_id, question_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (question_id) REFERENCES questions(id)
);

-- 创建题解表
CREATE TABLE solutions (
    id INT AUTO_INCREMENT COMMENT '题解ID',
    question_id INT NOT NULL COMMENT '题目ID',
    user_id INT NOT NULL COMMENT '用户ID',
    title VARCHAR(255) NOT NULL COMMENT '题解标题',
    content TEXT NOT NULL COMMENT '题解内容',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- 创建评论表
CREATE TABLE comments (
    id INT AUTO_INCREMENT COMMENT '评论ID',
    solution_id INT NOT NULL COMMENT '题解ID',
    user_id INT NOT NULL COMMENT '用户ID',
    content TEXT NOT NULL COMMENT '评论内容',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (solution_id) REFERENCES solutions(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- 创建索引
CREATE INDEX idx_users_open_id ON users(open_id);
CREATE INDEX idx_tags_category_id ON tags(category_id);
CREATE INDEX idx_questions_category_id ON questions(category_id);
CREATE INDEX idx_question_tags_question_id ON question_tags(question_id);
CREATE INDEX idx_question_tags_tag_id ON question_tags(tag_id);
CREATE INDEX idx_user_questions_user_id ON user_questions(user_id);
CREATE INDEX idx_user_questions_question_id ON user_questions(question_id);
CREATE INDEX idx_solutions_question_id ON solutions(question_id);
CREATE INDEX idx_solutions_user_id ON solutions(user_id);
CREATE INDEX idx_comments_solution_id ON comments(solution_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);

