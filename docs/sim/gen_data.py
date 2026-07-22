import random

random.seed(0)

count = 100
delta = 2

supports_answers_en = ["Yes", "No"]
supports_answers_fr = ["Oui", "Non"]
grade_answers_en = ["Grade 12", "Grade 11", "Grade 10"]
grade_answers_fr = ["12e année", "11e année", "10e année"]


def random_distance(answers, starter_idx):
    idx = starter_idx + random.randint(-delta, delta)
    if idx < 0:
        idx = 0
    if idx >= len(answers):
        idx = len(answers) - 1
    return answers[idx]


with open('sample_en.csv', 'a') as file_en:
    with open('sample_fr.csv', 'a') as file_fr:
        file_en.truncate(0)
        file_en.write('hw_hours,supports,grade,stress')
        file_fr.truncate(0)
        file_fr.write('dv_heures,supports,année,stresse')
        i = 0
        while i < count:
            stress = random.randint(1, 5)
            hours = max(round(4 * (stress / 2 + random.uniform(-1, 1))) / 4, 0.0)
    
            file_en.write('\n' + str(hours) + ',' + random_distance(supports_answers_en, stress) + ',' + random_distance(grade_answers_en, stress) + ',' + str(stress))
            file_fr.write('\n' + str(hours) + ',' + random_distance(supports_answers_fr, stress) + ',' + random_distance(grade_answers_fr, stress) + ',' + str(stress))
    
            i += 1
