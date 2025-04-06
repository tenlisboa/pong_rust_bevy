# 🕹️ Projeto 1 – Pong com Esteroides (Rust + Bevy)

> Primeiro projeto do plano de estudos com foco em aprender os fundamentos de Rust e Bevy com um jogo 2D simples, usando ECS, entrada de usuário, movimentação e lógica de jogo.

---

## 🎯 Objetivo

- Aprender Rust na prática, entendendo ownership, borrow checker, módulos e enums.
- Dominar o básico da Bevy Engine e sua arquitetura ECS.
- Criar um jogo funcional e modular com estrutura limpa e extensível.

---

## 🧠 Conceitos-Chave

- Bevy App lifecycle
- ECS (Entities, Components, Systems)
- Transform e Sprite
- Input do teclado
- Colisões simples
- Sistema de pontuação
- HUD com texto

---

## 🗂️ Estrutura Sugerida do Projeto

pong_rust_bevy/
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── game.rs
│ ├── components.rs
│ ├── systems/
│ │ ├── input.rs
│ │ ├── movement.rs
│ │ ├── collision.rs
│ │ └── scoring.rs
│ └── config.rs

---

## ✅ Funcionalidades Básicas

| Feature                  | Status |
| ------------------------ | ------ |
| Janela com câmera 2D     | 🔲     |
| Paddle do jogador        | 🔲     |
| Bola com movimento       | 🔲     |
| Input de movimento (W/S) | 🔲     |
| Colisão bola/paddle      | 🔲     |
| Colisão com paredes      | 🔲     |
| Sistema de pontuação     | 🔲     |
| UI com placar            | 🔲     |
| Reinício da partida      | 🔲     |
| Sons (rebate, ponto)     | 🔲     |

---

## 🚧 Milestones

### 📍 Milestone 1 – Setup inicial

- [x] Criar projeto com `cargo new`
- [ ] Adicionar Bevy ao `Cargo.toml`
- [ ] Janela + câmera 2D funcionando
- [ ] Sprite do paddle visível na tela

### 📍 Milestone 2 – Movimento e colisão

- [ ] Sistema de input funcionando (W/S)
- [ ] Movimento do paddle
- [ ] Bola se movendo automaticamente
- [ ] Colisão com paddle e bordas

### 📍 Milestone 3 – Pontuação e HUD

- [ ] Contar pontos (quando bola passa por paddle)
- [ ] Exibir pontuação na tela (HUD com texto)
- [ ] Reset da bola após ponto

### 📍 Milestone 4 – Polimento (opcional)

- [ ] Sons básicos usando `bevy_kira_audio`
- [ ] Efeitos visuais (trail, mudança de cor)
- [ ] Splash screen / menu inicial simples

---

## 📚 Recursos Úteis

- 📘 [The Rust Book](https://doc.rust-lang.org/book/)
- 🧪 [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
- 🧰 [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)

---

## 💡 Dicas

- Comece simples: um paddle, uma bola.
- Teste cada sistema separadamente (input, física, colisão).
- Use `println!()` para depurar no começo.
- Modularize os sistemas cedo — isso te ajuda a escalar o projeto depois.

---

## 🧾 Diário de Desenvolvimento (exemplo)

**Dia 1 – Setup**

- [x] Criei o projeto com cargo
- [x] Janela abre com background preto
- [x] Paddle aparece centralizado
- Próximo: movimento via input

---

## 🔗 Licença

MIT
