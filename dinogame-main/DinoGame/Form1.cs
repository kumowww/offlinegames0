using System;
using System.Collections.Generic;
using System.Drawing;
using System.Windows.Forms;

namespace DinoGame
{
    public partial class DinoGameForm : Form
    {
        System.Windows.Forms.Timer gameTimer = new System.Windows.Forms.Timer();
        Rectangle dino = new Rectangle(50, 220, 44, 47);
        bool isJumping = false;
        int jumpSpeed = 0;
        int force = 12; 
        int score = 0;
        List<Rectangle> obstacles = new List<Rectangle>();
        Random rand = new Random();
        bool gameOver = false;

        public DinoGameForm()
        {
            this.DoubleBuffered = true;
            this.Width = 800;
            this.Height = 300;
            this.Text = "Dino Game";
            this.BackColor = Color.Black;
            this.FormBorderStyle = FormBorderStyle.FixedSingle;
            this.MaximizeBox = false;

            gameTimer.Interval = 20;
            gameTimer.Tick += GameTick;
            this.Paint += DrawGame;
            this.KeyDown += OnKeyDown;
            this.KeyUp += OnKeyUp; 
            StartGame();
        }

        void StartGame()
        {
            dino.Y = 220;
            isJumping = false;
            jumpSpeed = 0;
            force = 12;
            score = 0;
            obstacles.Clear();
            gameOver = false;
            AddObstacle();
            gameTimer.Start();
        }

        void GameTick(object sender, EventArgs e)
        {
            if (gameOver) return;

            // Dino jump logic - Dino-Sprunglogik
            if (isJumping)
            {
                dino.Y -= jumpSpeed;
                jumpSpeed -= 1;
                if (dino.Y >= 220)
                {
                    dino.Y = 220;
                    isJumping = false;
                }
            }

            // Move obstacles - Hindernisse verschieben
            for (int i = 0; i < obstacles.Count; i++)
            {
                Rectangle obs = obstacles[i];
                obs.X -= 10;
                obstacles[i] = obs;
            }

            // Remove off-screen obstacles and add new ones - Entfernen Hindernisse außerhalb des Bildschirms und fügen neue hinzu
            if (obstacles.Count > 0 && obstacles[0].X < -60)
            {
                obstacles.RemoveAt(0);
                AddObstacle();
                score++;
            }

            // Collision detection - Kollisionserkennung
            foreach (var obs in obstacles)
            {
                if (dino.IntersectsWith(obs))
                {
                    gameTimer.Stop();
                    gameOver = true;
                    Invalidate();
                    return;
                }
            }

            Invalidate();
        }

        void AddObstacle()
        {
            int height = rand.Next(30, 50);
            int width = rand.Next(20, 30);
            int y = 247 - height;
            int x = 800 + rand.Next(0, 100);
            obstacles.Add(new Rectangle(x, y, width, height));
        }

        void DrawGame(object sender, PaintEventArgs e)
        {
            Graphics g = e.Graphics;

            // Draw ground - Boden zeichnen
            g.FillRectangle(Brushes.Gray, 0, 267, 800, 5);


            // Draw dino (simple representation) - Dino zeichnen (einfache Darstellung)
            g.FillRectangle(Brushes.White, dino);

            // Draw obstacles - Hindernisse zeichnen
            foreach (var obs in obstacles)
            {
                g.FillRectangle(Brushes.DarkGreen, obs);
            }

            // Draw score - Unentschieden
            g.DrawString("Score: " + score, new Font("Arial", 16), Brushes.White, 650, 10);
            if (gameOver)
            {
                g.DrawString("GAME OVER", new Font("Georgia", 28, FontStyle.Bold), Brushes.Red, 300, 100);
                g.DrawString(" Press Space to Restart", new Font("Georgia", 18), Brushes.White, 300, 150);
            }
        }

        void OnKeyDown(object sender, KeyEventArgs e)
        {
            if (e.KeyCode == Keys.Space)
            {
                if (!isJumping && !gameOver)
                {
                    isJumping = true;
                    jumpSpeed = force;
                }
                else if (gameOver)
                {
                    StartGame();
                }
            }
        }

        void OnKeyUp(object sender, KeyEventArgs e)
        {
            // No action needed for key up in this simple game - In diesem einfachen Spiel ist zum Eingeben Taste keine Aktion erforderlich...
        }


    }
}