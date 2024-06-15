import pygame, os, time
pygame.init()

blocksize = 50
screen = pygame.display.set_mode((16 * blocksize, 18 * blocksize))
clock = pygame.time.Clock()
font = pygame.font.Font("freesansbold.ttf", 100)
running = True
valid_unicode = "0123456789"

textures = {}
for texture in os.listdir("assets/tiles"):
    textures[int(texture.split(".")[0])] = pygame.transform.scale(pygame.image.load("assets/tiles/%s" % texture).convert_alpha(), (blocksize, blocksize)) 

def event_loop():
    global running
    for event in pygame.event.get():
        if event.type == pygame.QUIT: running = False
        if event.type == pygame.MOUSEBUTTONDOWN: 
            if event.button == 1: return "click"
        if event.type == pygame.MOUSEWHEEL:
            print("SCROLLED: %s" % event.y)
            return "scroll:%s" % event.y
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_RETURN:
                return "enter"
            if event.key == pygame.K_BACKSPACE:
                return "backspace"
            if event.key == pygame.K_SPACE:
                return "space"
            if event.unicode in valid_unicode:
                return "key:%s" % event.unicode

class Button:
    def __init__(self, text, relx, rely):
        self.text = text
        self.relx = relx
        self.rely = rely
        self.text_surf = font.render("[%s]" % self.text, False, (0, 30, 150))
        self.surf = pygame.surface.Surface((self.text_surf.get_width() + 10, self.text_surf.get_height() + 10))
        
    def draw(self):
        tlx = (16 * blocksize) * self.relx - self.surf.get_width() / 2
        tly = (16 * blocksize) * self.rely - self.surf.get_height() / 2
        brx = (16 * blocksize) * self.relx + self.surf.get_width() / 2
        bry = (16 * blocksize) * self.rely + self.surf.get_height() / 2
        hover = False
        mx, my = pygame.mouse.get_pos()
        if mx >= tlx and mx <= brx:
            if my >= tly and my <= bry:
                hover = True
        if not hover:
            self.surf.fill((0, 15, 90))
        else:
            self.surf.fill((0, 10, 60))
        self.surf.blit(self.text_surf, (5, 5))
        screen.blit(self.surf, (tlx, tly))
        return hover

def multi_choice(a, b, c):
    a_b= Button(a.upper(), 0.5, 0.3)
    b_b = Button(b.upper(), 0.5, 0.45)
    c_b = Button(c.upper(), 0.5, 0.6)
    while running:
        dt = clock.tick(60)
        click = event_loop() == "click"
        screen.fill((255, 255, 255))
        if a_b.draw() and click: return a.lower()
        if b_b.draw() and click: return b.lower()
        if c_b.draw() and click: return c.lower()
        pygame.display.update()

def get_input(prompt):
    prompt_text = font.render("[%s]" % prompt, False, (0, 30, 150))
    tlx = 8 * blocksize - prompt_text.get_width() / 2
    tly = 50 - prompt_text.get_height() / 2
    submit_button = Button("SUBMIT", 0.5, 0.8)
    text = ""
    while running:
        dt = clock.tick(60)
        screen.fill((255, 255, 255))
        screen.blit(prompt_text, (tlx, tly))
        event = event_loop()
        if event != None:
            if event[0:3] == "key":
                text += event.split(":")[1]
        if event == "backspace":
            text = text[0:len(text)-1]
        if event == "enter" or submit_button.draw() and event == "click":
            return text
        t = font.render("[%s]" % text, False, (0, 30, 150))
        ttlx = 8 * blocksize - t.get_width() / 2
        ttly = 150 - t.get_height() / 2
        screen.blit(t, (ttlx, ttly))
        pygame.display.update()

def alert(alert, duration):
    end = time.time() + duration
    alert = Button(alert, 0.5, 0.5)
    while time.time() < end:
        click = event_loop() == "click"
        screen.fill((255, 255, 255))
        dt = clock.tick(60)
        if alert.draw() and click:
            break
        pygame.display.update()

def clamp(n, min, max):
    if n < min: return min
    if n > max: return max
    return n

def edit(tilemap):
    global running
    surf = pygame.surface.Surface((blocksize, blocksize))
    surf.fill((255, 255, 255))
    surf.set_alpha(100)
    t = 1
    flash_texture = None
    flash_time = 0
    while running:
        screen.fill((255, 255, 255))
        dt = clock.tick(60)
        event = event_loop()
        if event == "enter":
            action = multi_choice("SAVE", "DISCARD", "RESUME")
            if action == "save":
                return tilemap
            if action == "discard":
                return None
        if event != None:
            if event[0:3] == "scr":
                t += int(event.split(":")[1])
                t = clamp(t, 1, 7)
                flash_texture = textures[t]
                flash_time = 500
                print(t)
        for row in range(len(tilemap)):
            for tile in range(len(tilemap[0])):
                screen.blit(textures[tilemap[row][tile]], (tile * blocksize, row * blocksize))
        mx, my = pygame.mouse.get_pos()
        ix = int(mx/blocksize)
        iy = int(my/blocksize)
        ix = clamp(ix, 0, 15)
        iy = clamp(iy, 0, 15)
        if pygame.mouse.get_pressed()[0]:
            tilemap[iy][ix] = t
        flash_time -= dt
        if flash_time < 0:
            flash_time = 0
            flash_texture = None
        else:
            if flash_texture != None:
                screen.blit(pygame.transform.scale(flash_texture, (flash_texture.get_width() * 2, flash_texture.get_height() * 2)), (mx, my - blocksize * 3))
        screen.blit(surf, (ix * blocksize, iy * blocksize))
        pygame.display.update()

def save(tilemap, path):
    if tilemap != None:
        with open(path, "w") as dest_file:
            dest_file.write("\n".join([" ".join([str(t) for t in row]) for row in tilemap]))

while running:
    dt = clock.tick(60)
    next = multi_choice("NEW", "EDIT", "EXIT")
    if next == "exit":
        running = False
    if next == "edit":
        tilemap_id = get_input("ENTER ID")
        tilemap_filename = "assets/levels/%s.tilemap" % tilemap_id
        full_path = os.path.join(os.getcwd(), tilemap_filename)
        if os.path.exists(full_path):
            with open(full_path, "r") as file:
                lines = [[int(y) for y in x.strip("\n").split(" ")] for x in file.readlines()]
            save(edit(lines), full_path)
        else:
            alert("INVALID", 2)
    if next == "new":
        next = multi_choice("CUSTOM", "AUTO", "BACK")
        if next == "back":
            pass
        else:
            tilemap_id = "0"
            if next == "custom":
                tilemap_id = get_input("ENTER ID")
            elif next == "auto":
                found = False
                count = 1
                while not found:
                    if not os.path.exists(os.path.join(os.getcwd(), "assets/levels/%s.tilemap" % count)):
                        break
                    count += 1
                tilemap_id = str(count)
            tilemap_filename = "assets/levels/%s.tilemap" % tilemap_id
            full_path = os.path.join(os.getcwd(), tilemap_filename)
            if not os.path.exists(full_path):
                alert("NEW <%s>" % tilemap_id, 0.1)
                preset = multi_choice("EMPTY", "FILLED", "BORDER")
                with open("assets/presets/%s.tilemap" % preset, "r") as file:
                    lines = [[int(y) for y in x.strip("\n").split(" ")] for x in file.readlines()]
                save(edit(lines), full_path)
            else:
                alert("PATH EXISTS", 2)

pygame.quit()