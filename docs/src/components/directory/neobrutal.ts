import catalog from "../../../../registries/neobrutal/registry.json";
import { type Manifest } from "../blocks/block-copy";
import { GIT } from "@/lib/site";

const manifests = import.meta.glob<Manifest>(["./*.json", "!./registry.json"], {
  base: "../../../../registries/neobrutal",
  eager: true,
  import: "default",
});
const sourcePath = "registries/neobrutal";
const source = `https://github.com/${GIT.user}/${GIT.repo}/blob/${GIT.branch}/${sourcePath}`;

export const neobrutal = {
  ...catalog,
  items: catalog.items.map(({ manifest: filename, kind }) => {
    const manifest = manifests[`./${filename}`];
    if (!manifest) throw new Error(`Missing registry item ${filename}`);
    const slug = filename.replace(/\.json$/, "") as ItemSlug;
    const login = slug === "login";
    return {
      name: manifest.name,
      files: manifest.files.filter((file) => file.target.endsWith(".rs")),
      slug,
      kind,
      sourcePath,
      source,
      command: `gpui-deck add ${source}/${filename}`,
      poster: `/images/registries/neobrutal/${slug}`,
      posterWidth: 760,
      posterHeight: login ? 520 : 400,
      mobileHeight: login ? 520 : undefined,
    };
  }),
};

const copy = {
  en: {
    directory: "Registry directory",
    open: "Explore registry",
    original: "Original design system",
    guide: "Installation guide",
    items: "In this registry",
    block: "Block",
    component: "Component",
    titles: { login: "Login", button: "Button", input: "Input" },
    descriptions: {
      login:
        "An email and password form with local validation. Includes the button, input, and palette.",
      button: "A raised action with a solid shadow and distinct focus and pressed states.",
      input: "A labeled text field with a bold outline and a visible focus ring.",
    },
  },
  ko: {
    directory: "레지스트리",
    open: "레지스트리 둘러보기",
    original: "원본 디자인 시스템",
    guide: "설치 가이드",
    items: "수록 항목",
    block: "블록",
    component: "컴포넌트",
    titles: { login: "로그인", button: "버튼", input: "입력 필드" },
    descriptions: {
      login:
        "이메일과 비밀번호를 검사하는 로그인 폼입니다. 버튼과 입력 필드, 팔레트가 함께 설치됩니다.",
      button: "그림자 위에 떠 있는 버튼입니다. 키보드로 포커스하거나 누르면 모양이 달라집니다.",
      input: "레이블과 굵은 테두리가 있는 입력 필드입니다. 편집할 때 포커스 링이 표시됩니다.",
    },
  },
  ja: {
    directory: "レジストリ一覧",
    open: "レジストリを見る",
    original: "元のデザインシステム",
    guide: "インストールガイド",
    items: "収録アイテム",
    block: "ブロック",
    component: "コンポーネント",
    titles: { login: "ログイン", button: "ボタン", input: "入力フィールド" },
    descriptions: {
      login:
        "メールアドレスとパスワードを検証するフォーム。ボタンと入力欄、パレットも一緒に追加されます。",
      button: "影に浮かぶボタン。フォーカス時と押下時に見た目が変わります。",
      input: "ラベルと太い縁取りのある入力欄。フォーカスするとリングが表示されます。",
    },
  },
  zh: {
    directory: "注册表目录",
    open: "浏览注册表",
    original: "原始设计系统",
    guide: "安装指南",
    items: "收录项目",
    block: "区块",
    component: "组件",
    titles: { login: "登录", button: "按钮", input: "输入框" },
    descriptions: {
      login: "在本地校验邮箱和密码的登录表单，同时安装按钮、输入框和配色。",
      button: "带有实心阴影的按钮，键盘聚焦和按下时各有不同的外观。",
      input: "带标签和粗描边的文本输入框，聚焦时显示清晰的焦点环。",
    },
  },
};

type ItemSlug = keyof typeof copy.en.titles;

export function registryCopy(lang: string) {
  return copy[lang as keyof typeof copy] ?? copy.en;
}
