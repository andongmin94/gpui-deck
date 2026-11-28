// These summaries describe interactions in the installed registry screens.
const cms = {
  en: "Search posts, edit drafts and switch an article between draft and published.",
  ko: "게시글을 검색하고 초안을 편집하거나 발행 상태를 바꿀 수 있습니다.",
  ja: "投稿を検索し、下書きを編集して公開状態を切り替えられます。",
  zh: "搜索文章、编辑草稿，并在草稿与已发布状态之间切换。",
};

const descriptions: Record<string, typeof cms> = {
  "blog-template": {
    en: "Filter stories by topic, search the archive and open an article to read it in full.",
    ko: "주제별로 글을 고르고 보관함을 검색해 본문을 읽을 수 있습니다.",
    ja: "話題で絞り込んだり記事を検索したりして、本文を読めます。",
    zh: "按主题筛选或搜索归档，打开文章阅读全文。",
  },
  "cms-template": cms,
  "dashboard-template": {
    en: "Search project tasks, filter by completion and check off work to update progress.",
    ko: "프로젝트 작업을 검색하고 완료 여부로 걸러 보거나 작업을 완료해 진행률을 갱신합니다.",
    ja: "プロジェクトのタスクを検索し、完了状態で絞り込み、チェックして進捗を更新できます。",
    zh: "搜索项目任务，按完成状态筛选，勾选任务即可更新进度。",
  },
  "landing-template": {
    en: "Explore product features, compare monthly and yearly pricing and choose a plan.",
    ko: "제품 기능을 살펴보고 월간·연간 요금을 비교해 플랜을 선택합니다.",
    ja: "製品の機能を確認し、月払いと年払いの料金を比較してプランを選べます。",
    zh: "浏览产品功能，比较月付和年付价格并选择套餐。",
  },
  "link-hub-template": {
    en: "Browse a creator’s work and writing links, open destinations and copy their contact email.",
    ko: "창작자의 작업과 글 링크를 주제별로 열고 연락처 이메일을 복사할 수 있습니다.",
    ja: "クリエイターの作品や記事へのリンクを開き、連絡先メールアドレスをコピーできます。",
    zh: "浏览创作者的作品和文章链接，打开目标页面并复制联系邮箱。",
  },
  login: {
    en: "Submit an email and password with Enter and see validation errors beside each field.",
    ko: "이메일과 비밀번호를 Enter로 제출하고 각 입력란에서 검증 오류를 확인합니다.",
    ja: "メールアドレスとパスワードを Enter で送信し、各入力欄でエラーを確認できます。",
    zh: "输入邮箱和密码，按回车提交并在各字段旁查看校验错误。",
  },
  "portfolio-template": {
    en: "Expand project case studies, browse studio services and find contact details.",
    ko: "프로젝트 사례를 펼쳐 보고 스튜디오 서비스와 연락처를 살펴봅니다.",
    ja: "プロジェクトの事例を展開し、スタジオのサービスと連絡先を確認できます。",
    zh: "展开项目案例，浏览工作室服务和联系方式。",
  },
  "template-analytics": {
    en: "Compare revenue and acquisition channels over 7, 14 or 30 days.",
    ko: "7일·14일·30일 단위로 매출과 유입 채널을 비교합니다.",
    ja: "7日・14日・30日の期間で売上と流入チャネルを比較できます。",
    zh: "按 7 天、14 天或 30 天比较收入与获客渠道。",
  },
  "template-blog": {
    en: "Search and sort the blog archive, then open a post to read the full article.",
    ko: "블로그 글 목록을 검색하고 정렬한 뒤 원하는 글의 본문을 읽습니다.",
    ja: "ブログ記事を検索・並べ替えして、選んだ記事の本文を読めます。",
    zh: "搜索博客归档并排序，再打开文章阅读全文。",
  },
  "template-blog-post": {
    en: "Start in a full article and return to the archive to find another post.",
    ko: "추천 글의 본문을 읽고 목록으로 돌아가 다른 글을 검색합니다.",
    ja: "注目記事の本文から読み始め、一覧に戻ってほかの記事を探せます。",
    zh: "从精选文章的正文开始阅读，返回归档查找其他文章。",
  },
  "template-cms": cms,
  "template-data-manager": {
    en: "Filter draft records, edit rows and bulk delete selected records after confirmation.",
    ko: "데이터 관리자에서 초안 필터로 레코드를 찾아 편집하거나 선택한 행을 일괄 삭제합니다.",
    ja: "下書きで絞り込み、レコード編集や選択した行の一括削除ができます。",
    zh: "按草稿状态筛选、编辑记录，确认后批量删除选中的行。",
  },
  "template-link-hub": {
    en: "Filter guides, code and contact links, then open a resource or copy the email address.",
    ko: "가이드·코드·연락처 링크를 걸러 열고 이메일 주소를 복사할 수 있습니다.",
    ja: "ガイド・コード・連絡先のリンクを絞り込み、開いたりメールアドレスをコピーしたりできます。",
    zh: "按指南、代码或联系信息筛选链接，打开资源或复制邮箱地址。",
  },
  "template-portfolio": {
    en: "Browse selected work, expand a design case study and contact the studio.",
    ko: "대표 작업을 살펴보고 디자인 사례를 펼치거나 스튜디오 연락처를 확인합니다.",
    ja: "代表作を閲覧し、デザイン事例を展開してスタジオの連絡先を確認できます。",
    zh: "浏览精选作品、展开设计案例，并查看工作室联系方式。",
  },
  "template-settings": {
    en: "Edit the public profile, preview changes and save notification preferences.",
    ko: "공개 프로필을 편집해 미리 보고 알림 설정을 저장합니다.",
    ja: "公開プロフィールの変更をプレビューし、通知設定とともに保存できます。",
    zh: "编辑并预览公开资料，保存通知偏好。",
  },
};

export function registryBlockDescription(slug: string, lang: string) {
  const copy = descriptions[slug];
  if (!copy) throw new Error(`Missing block description for ${slug}`);
  return copy[lang as keyof typeof copy] ?? copy.en;
}
